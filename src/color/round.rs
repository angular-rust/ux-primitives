use super::*;

pub trait Round {
    fn round(self) -> Self;
}

impl Round for HslColor {
    fn round(self) -> Self {
        HslColor {
            hue: self.hue.round(),
            saturation: self.saturation.round(),
            lightness: self.lightness.round(),
        }
    }
}

impl Round for HsvColor {
    fn round(self) -> Self {
        HsvColor {
            hue: self.hue.round(),
            saturation: self.saturation.round(),
            value: self.value.round(),
        }
    }
}

impl Round for CmykColor {
    fn round(self) -> Self {
        CmykColor {
            cyan: self.cyan.round(),
            magenta: self.magenta.round(),
            yellow: self.yellow.round(),
            key: self.key.round(),
        }
    }
}

impl Round for CmyColor {
    fn round(self) -> Self {
        CmyColor {
            cyan: self.cyan.round(),
            magenta: self.magenta.round(),
            yellow: self.yellow.round(),
        }
    }
}