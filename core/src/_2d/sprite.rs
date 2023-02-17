use rp2040_hal::{gpio::PinId, spi::SpiDevice};

use crate::{Display, Color};

pub struct Sprite<const W: usize, const H: usize>(pub [[u16;W];H]);

pub struct SpriteAnimation<const W: usize, const H: usize> {
    pub frames: &'static [Sprite<W, H>],
    pub frame: f32,
    pub speed: f32
}
impl<const W: usize, const H: usize> SpriteAnimation<W, H> {
    pub fn new(frames: &'static [Sprite<W, H>]) -> Self {
        Self {
            frames,
            frame: 0.,
            speed: 1.
        }
    }
}

impl<const W: usize, const H: usize, SPID: SpiDevice, RST: PinId, DC: PinId, LED: PinId> Display<W, H, SPID, RST, DC, LED> {
    pub fn draw_sprite<const SW: usize, const SH: usize>(&mut self, sprite: &Sprite<SW, SH>, x: isize, y: isize) {
        for py in 0..SH {
            for px in 0..SW {
                self.set_pixel_color(px as isize + x, py as isize + y, sprite.0[py][px]);
            }
        }
    }
    pub fn draw_sprite_darken<const SW: usize, const SH: usize>(&mut self, sprite: &Sprite<SW, SH>, x: isize, y: isize, a: f32) {
        for py in 0..SH {
            for px in 0..SW {
                let mut color = Color::from_565(sprite.0[py][px]);
                color.darken(a);
                self.set_pixel_color(px as isize + x, py as isize + y, color.c);
            }
        }
    }
    pub fn draw_sprite_anim<const SW: usize, const SH: usize>(&mut self, sprite_anim: &mut SpriteAnimation<SW, SH>, x: isize, y: isize) {
        let frames_length = sprite_anim.frames.len();
        if frames_length == 0 { return }
        if sprite_anim.frame as usize >= frames_length {
            sprite_anim.frame = 0.
        }
        self.draw_sprite(&sprite_anim.frames[sprite_anim.frame as usize], x, y);
        sprite_anim.frame += sprite_anim.speed;
    }
}