#![no_std] #![no_main]
use rp_pico::entry;
use panic_halt as _;

use rp_pico::hal::{self, pac::{self, interrupt}, Clock};
use embedded_hal::digital::v2::OutputPin;
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;
use rp_spi_tft::{Orientation, Display};
use fugit::RateExtU32;

mod utils;
use utils::*;
#[allow(unused)]
mod assets;

static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

static mut SERIAL_BUFF: SpinLock<SerialBuff> = SpinLock::new(SerialBuff::new());

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog).ok().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, hal::Sio::new(pac.SIO).gpio_bank0, &mut pac.RESETS);

    unsafe {
        let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS
        ));
        USB_BUS = Some(usb_bus);
        let usb_bus = USB_BUS.as_ref().unwrap();
        USB_SERIAL = Some(SerialPort::new(&usb_bus));
        USB_DEVICE = Some(
            UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("rp-spi-tft")
                .product("terminal")
                .serial_number("0")
                .device_class(2)
                .build()
        );
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let _spi_sclk = pins.gpio6.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio7.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_miso = pins.gpio4.into_mode::<hal::gpio::FunctionSpi>();
    let mut d = Display::<160, 128, _, _, _, _>::new(
        hal::Spi::<_, _, 8>::new(pac.SPI0).init(
            &mut pac.RESETS, clocks.peripheral_clock.freq(), 16.MHz(), &embedded_hal::spi::MODE_0),
        pins.gpio5.into_push_pull_output(),  // rst
        pins.gpio3.into_push_pull_output(),  // dc
        pins.gpio2.into_push_pull_output()   // led
    );
    d.init(&mut delay);
    d.set_orientation(Orientation::LandscapeSwapped);
    d.led.set_high().unwrap();
    d.led.set_drive_strength(rp2040_hal::gpio::OutputDriveStrength::TwoMilliAmps);
    d.font = &assets::fira_code_regular_nerd_font_complete::F;
    d.color = rp_spi_tft::Color::WHITE;

    let mut terminal_buff = SerialBuff::new();
    loop {
        d.clear();

        unsafe { SERIAL_BUFF.copy_to(&mut terminal_buff) }

        for i in 0..terminal_buff.length {
            d.write_char(terminal_buff.data[i] as char)
        }

        d.draw();
    }
}

#[allow(non_snake_case)]
#[interrupt]
fn USBCTRL_IRQ() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if usb_dev.poll(&mut [serial]) {

        let mut local_buff = SerialBuff::new().data;
        match serial.read(&mut local_buff) {
            Ok(0) | Err(_) => {}
            Ok(count) => {
                let data = &local_buff[..count];

                serial.write(data).ok();

                let mut serial_buff = unsafe { SERIAL_BUFF.lock() };

                if data == b"cls" { // clear terminal
                    serial_buff.length = 0;
                } else {
                    for d in data {
                        serial_buff.append_data(*d)
                    }
                }
                
                unsafe { SERIAL_BUFF.unlock(serial_buff) }
            }
        }
        
    }
}