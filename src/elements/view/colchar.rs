use std::fmt::Debug;

#[derive(Copy)]
pub struct ColChar {
    pub fill_char: char,
    pub modifier: Modifier,
}

impl ColChar {
    pub const SOLID: Self = Self {
        fill_char: '█',
        modifier: Modifier::None,
    };
    pub const BACKGROUND: Self = Self {
        fill_char: '░',
        modifier: Modifier::None,
    };
    pub const EMPTY: Self = Self {
        fill_char: ' ',
        modifier: Modifier::None,
    };

    pub fn new(fill_char: char, modifier: Modifier) -> Self {
        Self {
            fill_char,
            modifier,
        }
    }

    pub fn render(&self) -> String {
        match self.modifier {
            Modifier::None => String::from(self.fill_char),
            _ => format!(
                "{}{}{}",
                self.modifier.as_string(),
                self.fill_char,
                Modifier::END.as_string()
            ),
        }
    }
}

impl Clone for ColChar {
    fn clone(&self) -> Self {
        Self {
            fill_char: self.fill_char,
            modifier: self.modifier,
        }
    }
}

impl Debug for ColChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Colchar(\"{}\")", self.render())
    }
}

#[derive(Copy)]
pub enum Modifier {
    Coded(u8),
    Colour { r: u8, g: u8, b: u8 },
    None,
}

impl Modifier {
    pub const END: Self = Self::Coded(0);
    pub const BOLD: Self = Self::Coded(1);
    pub const LIGHT: Self = Self::Coded(2);
    pub const ITALIC: Self = Self::Coded(3);
    pub const UNDERLINE: Self = Self::Coded(4);
    pub const INVERTED: Self = Self::Coded(7);
    pub const CROSSED: Self = Self::Coded(9);
    pub const RED: Self = Self::Coded(31);
    pub const GREEN: Self = Self::Coded(32);
    pub const YELLOW: Self = Self::Coded(33);
    pub const BLUE: Self = Self::Coded(34);
    pub const PURPLE: Self = Self::Coded(35);
    pub const CYAN: Self = Self::Coded(36);

    pub fn as_string(&self) -> String {
        match self {
            Self::Coded(code) => format!("\x1b[{}m", code),
            Self::Colour { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Self::None => String::new(),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Colour { r, g, b }
    }

    pub fn from_hsv(h: u8, s: u8, v: u8) -> Self {
        let h = h as f32 / 255.0;
        let s = s as f32 / 255.0;
        let v = v as f32 / 255.0;

        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1.0 - f * s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        let (r, g, b) = [
            (v, t, p),
            (q, v, p),
            (p, v, t),
            (p, q, v),
            (t, p, v),
            (v, p, q),
        ][(i % 6.0).floor() as usize];

        Self::Colour {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }
}

impl Clone for Modifier {
    fn clone(&self) -> Self {
        match self {
            Self::Coded(code) => Self::Coded(*code),
            Self::Colour { r, g, b } => Self::Colour {
                r: *r,
                g: *g,
                b: *b,
            },
            Self::None => Self::None,
        }
    }
}
