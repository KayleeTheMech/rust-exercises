use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl std::fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Yellow => write!(f, "Yellow"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::White => write!(f, "White"),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => return color.to_string(),
        Err(_) => return "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
