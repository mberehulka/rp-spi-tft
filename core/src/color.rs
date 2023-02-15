pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub brightness: [u16;256]
}
impl Color {
    pub fn new(mut r: f32, mut g: f32, mut b: f32) -> Self {
        if r > 1. { r = 1. } else if r < 0. { r = 0. }
        if g > 1. { g = 1. } else if g < 0. { g = 0. }
        if b > 1. { b = 1. } else if b < 0. { b = 0. }
        r *= 31.;
        g *= 63.;
        b *= 31.;
        let mut brightness = [0u16;256];
        for brt in 0..255 {
            let brtf = brt as f32 / 255.;
            let r = r * brtf;
            let g = g * brtf;
            let b = b * brtf;
            brightness[brt+1] =
                ((r as u16) << 11) +
                (((g as u16) << 5) & 0b00000_111111_00000) +
                ((b as u16) & 0b00000_000000_11111);
        }
        Self {
            r, g, b,
            brightness
        }
    }
}