#![no_std]
#![no_main]

#[link_section = ".boot2"] #[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

use core::fmt::Write;

use panic_halt as _;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut d = rp_spi_tft::Display::default();

    let mut counter = 0;

    loop {
        d.clear();

        write!(d, "Hello World ! {}\nTest", counter);

        counter += 1;

        d.draw();
    }
}