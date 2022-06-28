use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntEnum, Ord, PartialEq, PartialOrd, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut res: Vec<ResistorColor> = all::<ResistorColor>().collect();

    res.sort_unstable();
    res
}
