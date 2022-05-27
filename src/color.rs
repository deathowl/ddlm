#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    opacity: f32,
}

const fn rgb(red: f32, green: f32, blue: f32) -> Color {
    Color {
        red,
        green,
        blue,
        opacity: 1.0,
    }
}

impl Color {
    pub const BLACK: Self = rgb(0.0, 0.0, 0.0);
    pub const GRAY: Self = rgb(0.75, 0.75, 0.75);
    pub const WHITE: Self = rgb(1.0, 1.0, 1.0);
    pub const RED: Self = rgb(0.75, 0.25, 0.25);
    pub const YELLOW: Self = rgb(0.75, 0.75, 0.25);

    pub fn blend(&self, other: &Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);

        Self {
            red: self.red + ((other.red - self.red) * ratio),
            green: self.green + ((other.green - self.green) * ratio),
            blue: self.blue + ((other.blue - self.blue) * ratio),
            opacity: self.opacity + ((other.opacity - self.opacity) * ratio),
        }
    }

    pub fn as_argb8888(&self) -> u32 {
        let argb = [self.opacity, self.red, self.green, self.blue];
        u32::from_be_bytes(argb.map(|x| (x * 255.0) as u8))
    }
}
