use rp2040_hal::{gpio::PinId, spi::SpiDevice};
use core::fmt;

use crate::Display;

/// x = 5 bits, y = 5 bits, color = 6 bits
pub struct Font {
    pub line_height: isize,
    pub data: fn(char) -> (u8, &'static [u16])
}
pub const EMPTY_FONT: Font = Font {
    line_height: 0,
    data: |_| { (0, &[]) }
};

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> Display<W, H, SPID, RST, DC, LED> {
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.letter_cursor[0] = 0;
                self.letter_cursor[1] += self.line_height + self.font.line_height;
            }
            _ => {
                let (letter_width, data) = (self.font.data)(c);
                for pixel in data {
                    let x = ((pixel & 0b11111_00000_000000) >> 11) as isize + self.letter_cursor[0];
                    let y = ((pixel & 0b00000_11111_000000) >> 6) as isize + self.letter_cursor[1];
                    let a = (pixel & 0b00000_00000_111111) as f32 / 64.;
                    self.draw_pixel_blend(x, y, a);
                }
                self.letter_cursor[0] += self.letter_space + letter_width as isize;
            }
        }
    }
    pub fn set_cursor_position(&mut self, x: isize, y: isize) {
        self.letter_cursor = [x, y];
    }
}

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> fmt::Write for Display<W, H, SPID, RST, DC, LED> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)
        }
        Ok(())
    }
}