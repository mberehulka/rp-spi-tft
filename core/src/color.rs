#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub c: u16
}
impl Color {
    pub const BLACK: Self = Self::from_rgb_565(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb_565(31, 63, 31);
    pub const RED:   Self = Self::from_rgb_565(31, 0, 0);
    pub const GREEN: Self = Self::from_rgb_565(0, 63, 0);
    pub const BLUE:  Self = Self::from_rgb_565(0, 0, 31);

    #[inline(always)]
    pub const fn from_565(c: u16) -> Self {
        Self {
            r: (c >> 11) as u8,
            g: ((c & 0b00000_111111_00000) >> 5) as u8,
            b: (c & 0b00000_000000_11111) as u8,
            c
        }
    }
    /// from:
    /// ```
    /// 0 < r < 31
    /// 0 < g < 63
    /// 0 < b < 31
    /// ```
    #[inline(always)]
    pub const fn from_rgb_565(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b,
            c: convert(r, g, b)
        }
    }
    /// from:
    /// ```
    /// 0 < r < 255
    /// 0 < g < 255
    /// 0 < b < 255
    /// ```
    #[inline(always)]
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgb_565(
            ((r as f32 / 255.) * 31.) as u8,
            ((g as f32 / 255.) * 63.) as u8,
            ((b as f32 / 255.) * 31.) as u8
        )
    }

    /// 0 < a < 1
    #[inline(always)]
    pub fn sub(&mut self, color: u16) {
        let color = Self::from_565(color);
        self.r = if self.r < color.r { 0 } else { self.r - color.r };
        self.g = if self.g < color.g { 0 } else { self.g - color.g };
        self.b = if self.b < color.b { 0 } else { self.b - color.b };
        self.c = convert(self.r, self.g, self.b);
    }
    /// 0 < a < 1
    #[inline(always)]
    pub fn blend(&mut self, color: &Self, a: f32) {
        let ai = 1. - a;
        self.r = ((self.r as f32 * ai) + (color.r as f32 * a))as u8;
        self.g = ((self.g as f32 * ai) + (color.g as f32 * a))as u8;
        self.b = ((self.b as f32 * ai) + (color.b as f32 * a))as u8;
        self.c = convert(self.r, self.g, self.b);
    }

    #[inline(always)]
    pub fn darken(&mut self, a: f32) {
        self.r = (self.r as f32 * a) as u8;
        self.g = (self.g as f32 * a) as u8;
        self.b = (self.b as f32 * a) as u8;
        self.c = convert(self.r, self.g, self.b);
    }
}

#[inline(always)]
pub const fn convert(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16) << 11) + ((g as u16) << 5) + (b as u16)
}