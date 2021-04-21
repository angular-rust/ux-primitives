use super::*;

pub trait Round {
    fn round(self) -> Self;
}

impl Round for Rgb {
    fn round(self) -> Self {
        Rgb {
            red: self.red.round(),
            green: self.green.round(),
            blue: self.blue.round(),
            alpha: self.alpha.round(),
        }
    }
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

impl<C: Round> Round for Alpha<C> {
    fn round(self) -> Self {
        let (color, alpha) = self.split();
        Alpha::new(color.round(), alpha)
    }
}