use chrono::{Datelike, Local, Timelike};
use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;

pub fn date_as_hex() -> Hexadecimal {
    let dt = Local::now();
    let mut year = (dt.year() as u32).to_hex(2);
    let mut month = dt.month().to_hex(1);
    let mut day = dt.day().to_hex(1);
    let mut hour = dt.hour().to_hex(1);
    let mut minute = dt.minute().to_hex(1);
    let mut second = dt.second().to_hex(1);
    let mut nanosecond = dt.nanosecond().to_hex(1);
    year.append(&mut month);
    year.append(&mut day);
    year.append(&mut hour);
    year.append(&mut minute);
    year.append(&mut second);
    year.append(&mut nanosecond);
    return year;
}