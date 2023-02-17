#![no_std] #![no_main]
use rp_pico as _;
use panic_halt as _;

use core::fmt::Write;

mod assets;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut d = rp_spi_tft::Display::default();
    d.font = &assets::fira_code_regular_nerd_font_complete::F;
    d.color = rp_spi_tft::Color::BLACK;

    let mut sprite_anim = rp_spi_tft::SpriteAnimation::new(&assets::naruto_jiraya::S);

    loop {
        d.reset_cursor();

        d.draw_sprite_anim(&mut sprite_anim, 0, 0);

        write!(d, "Hello World !").ok();

        d.draw();
    }
}