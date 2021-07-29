//! Very nice color palette 

//#![rustfmt::skip]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[cfg(any(feature = "color_quantization",test))]
use lazy_static::lazy_static;
use crate::colorspace::Color;

pub const WHITE: Color = Color { red: 1., green: 1., blue: 1., alpha: 1. };
pub const BLACK: Color = Color { red: 0./255., green: 0./255., blue: 0./255., alpha: 1. };

pub const GRAY_0: Color = Color { red:248./255., green: 249./255., blue: 250./255., alpha: 1. };
pub const GRAY_1: Color = Color { red:241./255., green: 243./255., blue: 245./255., alpha: 1. };
pub const GRAY_2: Color = Color { red:233./255., green: 236./255., blue: 239./255., alpha: 1. };
pub const GRAY_3: Color = Color { red:222./255., green: 226./255., blue: 230./255., alpha: 1. };
pub const GRAY_4: Color = Color { red:206./255., green: 212./255., blue: 218./255., alpha: 1. };
pub const GRAY_5: Color = Color { red:173./255., green: 181./255., blue: 189./255., alpha: 1. };
pub const GRAY_6: Color = Color { red:134./255., green: 142./255., blue: 150./255., alpha: 1. };
pub const GRAY_7: Color = Color { red:73./255., green: 80./255., blue: 87./255., alpha: 1. };
pub const GRAY_8: Color = Color { red:52./255., green: 58./255., blue: 64./255., alpha: 1. };
pub const GRAY_9: Color = Color { red:33./255., green: 37./255., blue: 41./255., alpha: 1. };

pub const RED_0: Color = Color { red: 1., green: 245./255., blue: 245./255., alpha: 1. };
pub const RED_1: Color = Color { red: 1., green: 227./255., blue: 227./255., alpha: 1. };
pub const RED_2: Color = Color { red: 1., green: 201./255., blue: 201./255., alpha: 1. };
pub const RED_3: Color = Color { red: 1., green: 168./255., blue: 168./255., alpha: 1. };
pub const RED_4: Color = Color { red: 1., green: 135./255., blue: 135./255., alpha: 1. };
pub const RED_5: Color = Color { red: 1., green: 107./255., blue: 107./255., alpha: 1. };
pub const RED_6: Color = Color { red: 250./255., green: 82./255., blue: 82./255., alpha: 1. };
pub const RED_7: Color = Color { red: 240./255., green: 62./255., blue: 62./255., alpha: 1. };
pub const RED_8: Color = Color { red: 224./255., green: 49./255., blue: 49./255., alpha: 1. };
pub const RED_9: Color = Color { red: 201./255., green: 42./255., blue: 42./255., alpha: 1. };

pub const PINK_0: Color = Color { red: 1., green: 240./255., blue: 246./255., alpha: 1. };
pub const PINK_1: Color = Color { red: 1., green: 222./255., blue: 235./255., alpha: 1. };
pub const PINK_2: Color = Color { red: 252./255., green: 194./255., blue: 215./255., alpha: 1. };
pub const PINK_3: Color = Color { red: 250./255., green: 162./255., blue: 193./255., alpha: 1. };
pub const PINK_4: Color = Color { red: 247./255., green: 131./255., blue: 172./255., alpha: 1. };
pub const PINK_5: Color = Color { red: 240./255., green: 101./255., blue: 149./255., alpha: 1. };
pub const PINK_6: Color = Color { red: 230./255., green: 73./255., blue: 128./255., alpha: 1. };
pub const PINK_7: Color = Color { red: 214./255., green: 51./255., blue: 108./255., alpha: 1. };
pub const PINK_8: Color = Color { red: 194./255., green: 37./255., blue: 92./255., alpha: 1. };
pub const PINK_9: Color = Color { red: 166./255., green: 30./255., blue: 77./255., alpha: 1. };

pub const GRAPE_0: Color = Color { red: 248./255., green: 240./255., blue: 252./255., alpha: 1. };
pub const GRAPE_1: Color = Color { red: 243./255., green: 217./255., blue: 250./255., alpha: 1. };
pub const GRAPE_2: Color = Color { red: 238./255., green: 190./255., blue: 250./255., alpha: 1. };
pub const GRAPE_3: Color = Color { red: 229./255., green: 153./255., blue: 247./255., alpha: 1. };
pub const GRAPE_4: Color = Color { red: 218./255., green: 119./255., blue: 242./255., alpha: 1. };
pub const GRAPE_5: Color = Color { red: 204./255., green: 93./255., blue: 232./255., alpha: 1. };
pub const GRAPE_6: Color = Color { red: 190./255., green: 75./255., blue: 219./255., alpha: 1. };
pub const GRAPE_7: Color = Color { red: 174./255., green: 62./255., blue: 201./255., alpha: 1. };
pub const GRAPE_8: Color = Color { red: 156./255., green: 54./255., blue: 181./255., alpha: 1. };
pub const GRAPE_9: Color = Color { red: 134./255., green: 46./255., blue: 156./255., alpha: 1. };

pub const VIOLET_0: Color = Color { red: 243./255., green: 240./255., blue: 1., alpha: 1. };
pub const VIOLET_1: Color = Color { red: 229./255., green: 219./255., blue: 1., alpha: 1. };
pub const VIOLET_2: Color = Color { red: 208./255., green: 191./255., blue: 1., alpha: 1. };
pub const VIOLET_3: Color = Color { red: 177./255., green: 151./255., blue: 252./255., alpha: 1. };
pub const VIOLET_4: Color = Color { red: 151./255., green: 117./255., blue: 250./255., alpha: 1. };
pub const VIOLET_5: Color = Color { red: 132./255., green: 94./255., blue: 247./255., alpha: 1. };
pub const VIOLET_6: Color = Color { red: 121./255., green: 80./255., blue: 242./255., alpha: 1. };
pub const VIOLET_7: Color = Color { red: 112./255., green: 72./255., blue: 232./255., alpha: 1. };
pub const VIOLET_8: Color = Color { red: 103./255., green: 65./255., blue: 217./255., alpha: 1. };
pub const VIOLET_9: Color = Color { red: 95./255., green: 61./255., blue: 196./255., alpha: 1. };

pub const INDIGO_0: Color = Color { red: 237./255., green: 242./255., blue: 1., alpha: 1. };
pub const INDIGO_1: Color = Color { red: 219./255., green: 228./255., blue: 1., alpha: 1. };
pub const INDIGO_2: Color = Color { red: 186./255., green: 200./255., blue: 1., alpha: 1. };
pub const INDIGO_3: Color = Color { red: 145./255., green: 167./255., blue: 1., alpha: 1. };
pub const INDIGO_4: Color = Color { red: 116./255., green: 143./255., blue: 252./255., alpha: 1. };
pub const INDIGO_5: Color = Color { red: 92./255., green: 124./255., blue: 250./255., alpha: 1. };
pub const INDIGO_6: Color = Color { red: 76./255., green: 110./255., blue: 245./255., alpha: 1. };
pub const INDIGO_7: Color = Color { red: 66./255., green: 99./255., blue: 235./255., alpha: 1. };
pub const INDIGO_8: Color = Color { red: 59./255., green: 91./255., blue: 219./255., alpha: 1. };
pub const INDIGO_9: Color = Color { red: 54./255., green: 79./255., blue: 199./255., alpha: 1. };

pub const BLUE_0: Color = Color { red: 231./255., green: 245./255., blue: 1., alpha: 1. };
pub const BLUE_1: Color = Color { red: 208./255., green: 235./255., blue: 1., alpha: 1. };
pub const BLUE_2: Color = Color { red: 165./255., green: 216./255., blue: 1., alpha: 1. };
pub const BLUE_3: Color = Color { red: 116./255., green: 192./255., blue: 252./255., alpha: 1. };
pub const BLUE_4: Color = Color { red: 77./255., green: 171./255., blue: 247./255., alpha: 1. };
pub const BLUE_5: Color = Color { red: 51./255., green: 154./255., blue: 240./255., alpha: 1. };
pub const BLUE_6: Color = Color { red: 34./255., green: 139./255., blue: 230./255., alpha: 1. };
pub const BLUE_7: Color = Color { red: 28./255., green: 126./255., blue: 214./255., alpha: 1. };
pub const BLUE_8: Color = Color { red: 25./255., green: 113./255., blue: 194./255., alpha: 1. };
pub const BLUE_9: Color = Color { red: 24./255., green: 100./255., blue: 171./255., alpha: 1. };

pub const CYAN_0: Color = Color { red: 227./255., green: 250./255., blue: 252./255., alpha: 1. };
pub const CYAN_1: Color = Color { red: 197./255., green: 246./255., blue: 250./255., alpha: 1. };
pub const CYAN_2: Color = Color { red: 153./255., green: 233./255., blue: 242./255., alpha: 1. };
pub const CYAN_3: Color = Color { red: 102./255., green: 217./255., blue: 232./255., alpha: 1. };
pub const CYAN_4: Color = Color { red: 59./255., green: 201./255., blue: 219./255., alpha: 1. };
pub const CYAN_5: Color = Color { red: 34./255., green: 184./255., blue: 207./255., alpha: 1. };
pub const CYAN_6: Color = Color { red: 21./255., green: 170./255., blue: 191./255., alpha: 1. };
pub const CYAN_7: Color = Color { red: 16./255., green: 152./255., blue: 173./255., alpha: 1. };
pub const CYAN_8: Color = Color { red: 12./255., green: 133./255., blue: 153./255., alpha: 1. };
pub const CYAN_9: Color = Color { red: 11./255., green: 114./255., blue: 133./255., alpha: 1. };

pub const TEAL_0: Color = Color { red: 230./255., green: 252./255., blue: 245./255., alpha: 1. };
pub const TEAL_1: Color = Color { red: 195./255., green: 250./255., blue: 232./255., alpha: 1. };
pub const TEAL_2: Color = Color { red: 150./255., green: 242./255., blue: 215./255., alpha: 1. };
pub const TEAL_3: Color = Color { red: 99./255., green: 230./255., blue: 190./255., alpha: 1. };
pub const TEAL_4: Color = Color { red: 56./255., green: 217./255., blue: 169./255., alpha: 1. };
pub const TEAL_5: Color = Color { red: 32./255., green: 201./255., blue: 151./255., alpha: 1. };
pub const TEAL_6: Color = Color { red: 18./255., green: 184./255., blue: 134./255., alpha: 1. };
pub const TEAL_7: Color = Color { red: 12./255., green: 166./255., blue: 120./255., alpha: 1. };
pub const TEAL_8: Color = Color { red: 9./255., green: 146./255., blue: 104./255., alpha: 1. };
pub const TEAL_9: Color = Color { red: 8./255., green: 127./255., blue: 91./255., alpha: 1. };

pub const GREEN_0: Color = Color { red: 235./255., green: 251./255., blue: 238./255., alpha: 1. };
pub const GREEN_1: Color = Color { red: 211./255., green: 249./255., blue: 216./255., alpha: 1. };
pub const GREEN_2: Color = Color { red: 178./255., green: 242./255., blue: 187./255., alpha: 1. };
pub const GREEN_3: Color = Color { red: 140./255., green: 233./255., blue: 154./255., alpha: 1. };
pub const GREEN_4: Color = Color { red: 105./255., green: 219./255., blue: 124./255., alpha: 1. };
pub const GREEN_5: Color = Color { red: 81./255., green: 207./255., blue: 102./255., alpha: 1. };
pub const GREEN_6: Color = Color { red: 64./255., green: 192./255., blue: 87./255., alpha: 1. };
pub const GREEN_7: Color = Color { red: 55./255., green: 178./255., blue: 77./255., alpha: 1. };
pub const GREEN_8: Color = Color { red: 47./255., green: 158./255., blue: 68./255., alpha: 1. };
pub const GREEN_9: Color = Color { red: 43./255., green: 138./255., blue: 62./255., alpha: 1. };

pub const LIME_0: Color = Color { red: 244./255., green: 252./255., blue: 227./255., alpha: 1. };
pub const LIME_1: Color = Color { red: 233./255., green: 250./255., blue: 200./255., alpha: 1. };
pub const LIME_2: Color = Color { red: 216./255., green: 245./255., blue: 162./255., alpha: 1. };
pub const LIME_3: Color = Color { red: 192./255., green: 235./255., blue: 117./255., alpha: 1. };
pub const LIME_4: Color = Color { red: 169./255., green: 227./255., blue: 75./255., alpha: 1. };
pub const LIME_5: Color = Color { red: 148./255., green: 216./255., blue: 45./255., alpha: 1. };
pub const LIME_6: Color = Color { red: 130./255., green: 201./255., blue: 30./255., alpha: 1. };
pub const LIME_7: Color = Color { red: 116./255., green: 184./255., blue: 22./255., alpha: 1. };
pub const LIME_8: Color = Color { red: 102./255., green: 168./255., blue: 15./255., alpha: 1. };
pub const LIME_9: Color = Color { red: 92./255., green: 148./255., blue: 13./255., alpha: 1. };

pub const YELLOW_0: Color = Color { red: 1., green: 249./255., blue: 219./255., alpha: 1. };
pub const YELLOW_1: Color = Color { red: 1., green: 243./255., blue: 191./255., alpha: 1. };
pub const YELLOW_2: Color = Color { red: 1., green: 236./255., blue: 153./255., alpha: 1. };
pub const YELLOW_3: Color = Color { red: 1., green: 224./255., blue: 102./255., alpha: 1. };
pub const YELLOW_4: Color = Color { red: 1., green: 212./255., blue: 59./255., alpha: 1. };
pub const YELLOW_5: Color = Color { red: 252./255., green: 196./255., blue: 25./255., alpha: 1. };
pub const YELLOW_6: Color = Color { red: 250./255., green: 176./255., blue: 5./255., alpha: 1. };
pub const YELLOW_7: Color = Color { red: 245./255., green: 159./255., blue: 0./255., alpha: 1. };
pub const YELLOW_8: Color = Color { red: 240./255., green: 140./255., blue: 0./255., alpha: 1. };
pub const YELLOW_9: Color = Color { red: 230./255., green: 119./255., blue: 0./255., alpha: 1. };

pub const ORANGE_0: Color = Color { red: 1., green: 244./255., blue: 230./255., alpha: 1. };
pub const ORANGE_1: Color = Color { red: 1., green: 232./255., blue: 204./255., alpha: 1. };
pub const ORANGE_2: Color = Color { red: 1., green: 216./255., blue: 168./255., alpha: 1. };
pub const ORANGE_3: Color = Color { red: 1., green: 192./255., blue: 120./255., alpha: 1. };
pub const ORANGE_4: Color = Color { red: 1., green: 169./255., blue: 77./255., alpha: 1. };
pub const ORANGE_5: Color = Color { red: 1., green: 146./255., blue: 43./255., alpha: 1. };
pub const ORANGE_6: Color = Color { red: 253./255., green: 126./255., blue: 20./255., alpha: 1. };
pub const ORANGE_7: Color = Color { red: 247./255., green: 103./255., blue: 7./255., alpha: 1. };
pub const ORANGE_8: Color = Color { red: 232./255., green: 89./255., blue: 12./255., alpha: 1. };
pub const ORANGE_9: Color = Color { red: 217./255., green: 72./255., blue: 15./255., alpha: 1. };

#[cfg(any(feature = "color_quantization",test))]
lazy_static! {
    pub static ref PALETTE: Vec<Color> = vec!(
        BLACK, WHITE,
        GRAY_0, GRAY_1, GRAY_2, GRAY_3, GRAY_4, GRAY_5, GRAY_6, GRAY_7, GRAY_8, GRAY_9,
        RED_0, RED_1, RED_2, RED_3, RED_4, RED_5, RED_6, RED_7, RED_8, RED_9,
        PINK_0, PINK_1, PINK_2, PINK_3, PINK_4, PINK_5, PINK_6, PINK_7, PINK_8, PINK_9,
        GRAPE_0, GRAPE_1, GRAPE_2, GRAPE_3, GRAPE_4, GRAPE_5, GRAPE_6, GRAPE_7, GRAPE_8, GRAPE_9,
        VIOLET_0, VIOLET_1, VIOLET_2, VIOLET_3, VIOLET_4, VIOLET_5, VIOLET_6, VIOLET_7, VIOLET_8, VIOLET_9,
        INDIGO_0, INDIGO_1, INDIGO_2, INDIGO_3, INDIGO_4, INDIGO_5, INDIGO_6, INDIGO_7, INDIGO_8, INDIGO_9,
        BLUE_0, BLUE_1, BLUE_2, BLUE_3, BLUE_4, BLUE_5, BLUE_6, BLUE_7, BLUE_8, BLUE_9,
        CYAN_0, CYAN_1, CYAN_2, CYAN_3, CYAN_4, CYAN_5, CYAN_6, CYAN_7, CYAN_8, CYAN_9,
        TEAL_0, TEAL_1, TEAL_2, TEAL_3, TEAL_4, TEAL_5, TEAL_6, TEAL_7, TEAL_8, TEAL_9,
        GREEN_0, GREEN_1, GREEN_2, GREEN_3, GREEN_4, GREEN_5, GREEN_6, GREEN_7, GREEN_8, GREEN_9,
        LIME_0, LIME_1, LIME_2, LIME_3, LIME_4, LIME_5, LIME_6, LIME_7, LIME_8, LIME_9,
        YELLOW_0, YELLOW_1, YELLOW_2, YELLOW_3, YELLOW_4, YELLOW_5, YELLOW_6, YELLOW_7, YELLOW_8, YELLOW_9,
        ORANGE_0, ORANGE_1, ORANGE_2, ORANGE_3, ORANGE_4, ORANGE_5, ORANGE_6, ORANGE_7, ORANGE_8, ORANGE_9,
    );
}
