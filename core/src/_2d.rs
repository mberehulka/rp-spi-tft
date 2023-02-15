use rp2040_hal::{gpio::PinId, spi::SpiDevice};

use crate::Display;

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> Display<W, H, SPID, RST, DC, LED> {
    pub fn draw_rect(&mut self, x: isize, y: isize, w: isize, h: isize) {
        for y in y..(y + h) {
            for x in x..(x + w) {
                self.draw_pixel(x, y)
            }
        }
    }
}