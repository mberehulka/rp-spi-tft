use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin, prelude::_embedded_hal_blocking_spi_Write};
use rp2040_hal::{spi::{Enabled, SpiDevice}, gpio::{Output, PushPull, Pin, PinId, bank0}, Spi, Clock, pac::SPI0};
use fugit::RateExtU32;

use crate::{Instruction, Orientation, Color, Font, EMPTY_FONT};

pub struct Display<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> {
    spi: rp2040_hal::Spi<Enabled, SPID, 8>,
    rst: Pin<RST, Output<PushPull>>,
    dc: Pin<DC, Output<PushPull>>,
    pub led: Pin<LED, Output<PushPull>>,
    pub data: [[u16;W];H],
    pub color: Color,

    pub letter_space: isize,
    pub letter_cursor: [isize;2],
    pub letter_padding: [isize;2],
    pub word_space: isize,
    pub line_height: isize,
    pub font: &'static Font
}
impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> Display<W, H, SPID, RST, DC, LED> {
    pub fn new(
        spi: Spi<Enabled, SPID, 8>,
        rst: Pin<RST, Output<PushPull>>,
        dc: Pin<DC, Output<PushPull>>,
        led: Pin<LED, Output<PushPull>>
    ) -> Self {
        let color = Color::WHITE;
        Self {
            spi, rst, dc, led,
            data: [[0; W];H],
            color,

            letter_space: 1,
            letter_cursor: [0, 0],
            letter_padding: [0, 0],
            word_space: 0,
            line_height: 0,
            font: &EMPTY_FONT
        }
    }
    pub fn init(&mut self, delay: &mut impl DelayMs<u8>) {
        self.hard_reset(delay);
        self.write_command(Instruction::SWRESET, &[]);
        delay.delay_ms(200);
        self.write_command(Instruction::SLPOUT, &[]);
        delay.delay_ms(200);
        self.write_command(Instruction::FRMCTR1, &[0x01, 0x2C, 0x2D]);
        self.write_command(Instruction::FRMCTR2, &[0x01, 0x2C, 0x2D]);
        self.write_command(Instruction::FRMCTR3, &[0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]);
        self.write_command(Instruction::INVCTR, &[0x07]);
        self.write_command(Instruction::PWCTR1, &[0xA2, 0x02, 0x84]);
        self.write_command(Instruction::PWCTR2, &[0xC5]);
        self.write_command(Instruction::PWCTR3, &[0x0A, 0x00]);
        self.write_command(Instruction::PWCTR4, &[0x8A, 0x2A]);
        self.write_command(Instruction::PWCTR5, &[0x8A, 0xEE]);
        self.write_command(Instruction::VMCTR1, &[0x0E]);
        self.write_command(Instruction::INVOFF, &[]);
        self.write_command(Instruction::MADCTL, &[0x00]);
        self.write_command(Instruction::COLMOD, &[0x05]);
        self.write_command(Instruction::DISPON, &[]);
        delay.delay_ms(200);
    }
    pub fn hard_reset(&mut self, delay: &mut impl DelayMs<u8>) {
        self.rst.set_high().map_err(|_| ()).unwrap();
        delay.delay_ms(10);
        self.rst.set_low().map_err(|_| ()).unwrap();
        delay.delay_ms(10);
        self.rst.set_high().map_err(|_| ()).unwrap();
    }
    pub fn write_command(&mut self, command: Instruction, params: &[u8]) {
        self.dc.set_low().map_err(|_| ()).unwrap();
        self.spi.write(&[command as u8]).map_err(|_| ()).unwrap();
        if params.len() > 0 {
            self.start_data();
            self.write_data(params);
        }
    }
    pub fn start_data(&mut self) {
        self.dc.set_high().map_err(|_| ()).unwrap();
    }
    pub fn write_data(&mut self, data: &[u8]) {
        self.spi.write(data).map_err(|_| ()).unwrap();
    }
    pub fn write_word(&mut self, value: u16) {
        self.write_data(&value.to_be_bytes());
    }
    pub fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) {
        self.write_command(Instruction::CASET, &[]);
        self.start_data();
        self.write_word(sx);
        self.write_word(ex);
        self.write_command(Instruction::RASET, &[]);
        self.start_data();
        self.write_word(sy);
        self.write_word(ey);
    }
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.write_command(Instruction::MADCTL, &[orientation as u8]);
    }
    pub fn reset_cursor(&mut self) {
        self.letter_cursor[0] = 0;
        self.letter_cursor[1] = self.line_height;
    }
    pub fn clear(&mut self) {
        self.reset_cursor();
        for y in 0..H {
            for x in 0..W {
                self.data[y][x] = 0
            }
        }
    }
    pub fn draw(&mut self) {
        self.set_address_window(0, 0, W as u16 - 1, H as u16 - 1);
        self.write_command(Instruction::RAMWR, &[]);
        self.start_data();
        for y in 0..H {
            for x in 0..W {
                self.spi.write(&self.data[y][x].to_be_bytes()).map_err(|_| ()).unwrap();
            }
        }
    }

    #[inline(always)]
    pub fn draw_pixel(&mut self, x: isize, y: isize) {
        if x >= W as isize || x < 0 || y >= H as isize || y < 0 { return }
        self.data[y as usize][x as usize] = self.color.c
    }
    #[inline(always)]
    pub fn draw_pixel_blend(&mut self, x: isize, y: isize, a: f32) {
        if x >= W as isize || x < 0 || y >= H as isize || y < 0 { return }
        let mut color = Color::from_565(self.data[y as usize][x as usize]);
        color.blend(&self.color, a);
        self.data[y as usize][x as usize] = color.c;
    }

    #[inline(always)]
    pub fn set_pixel_color(&mut self, x: isize, y: isize, c: u16) {
        if x >= W as isize || x < 0 || y >= H as isize || y < 0 { return }
        self.data[y as usize][x as usize] = c
    }
}


impl Display<160, 128, SPI0, bank0::Gpio5, bank0::Gpio3, bank0::Gpio2> {
    pub fn default() -> Self {
        let mut pac = rp2040_hal::pac::Peripherals::take().unwrap();
        let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);
        let clocks = rp2040_hal::clocks::init_clocks_and_plls(
            12_000_000u32, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog).ok().unwrap();
        let core = rp2040_hal::pac::CorePeripherals::take().unwrap();
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        let pins = rp2040_hal::gpio::Pins::new(
            pac.IO_BANK0, pac.PADS_BANK0, rp2040_hal::Sio::new(pac.SIO).gpio_bank0, &mut pac.RESETS);

        let _spi_sclk = pins.gpio6.into_mode::<rp2040_hal::gpio::FunctionSpi>();
        let _spi_mosi = pins.gpio7.into_mode::<rp2040_hal::gpio::FunctionSpi>();
        let _spi_miso = pins.gpio4.into_mode::<rp2040_hal::gpio::FunctionSpi>();
        let mut s = Self::new(
            rp2040_hal::Spi::<_, _, 8>::new(pac.SPI0).init(
                &mut pac.RESETS, clocks.peripheral_clock.freq(), 16.MHz(), &embedded_hal::spi::MODE_0),
            pins.gpio5.into_push_pull_output(),  // rst
            pins.gpio3.into_push_pull_output(),  // dc
            pins.gpio2.into_push_pull_output()   // led
        );
        s.init(&mut delay);
        s.set_orientation(Orientation::LandscapeSwapped);
        s.led.set_high().unwrap();
        s.led.set_drive_strength(rp2040_hal::gpio::OutputDriveStrength::TwoMilliAmps);
        s
    }
}