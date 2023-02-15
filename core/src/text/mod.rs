use rp2040_hal::{gpio::PinId, spi::SpiDevice};
use core::fmt;

use crate::Display;

#[rustfmt::skip]
pub mod font;

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> Display<W, H, SPID, RST, DC, LED> {
    #[inline(never)]
    pub fn draw_letter_pixel(&mut self, x: i8, y: i8, brt: u8) {
        let x = x as isize + self.letter_cursor[0];
        if x < 0 { return }
        let y = y as isize + self.letter_cursor[1];
        if y < 0 { return }
        let x = x as usize;
        if x >= W { return }
        let y = y as usize;
        if y >= W { return }
        self.data[y][x] = self.color.brightness[brt as usize]
    }
    pub fn set_cursor_position(&mut self, x: isize, y: isize) {
        self.letter_cursor = [x, y];
    }
    #[inline(never)]
    pub fn print_char(&mut self, c: char) -> u8 {
        font::draw_letter!(self, c)
    }
}

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> fmt::Write for Display<W, H, SPID, RST, DC, LED> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => {
                    self.letter_cursor[0] = 0;
                    self.letter_cursor[1] += self.line_height + font::LINE_HEIGHT;
                }
                _ => {
                    let letter_width = self.print_char(c);
                    self.letter_cursor[0] += self.letter_space + letter_width as isize;
                }
            }
        }
        Ok(())
    }
}