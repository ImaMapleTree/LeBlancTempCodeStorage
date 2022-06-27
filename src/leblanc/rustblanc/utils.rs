
use std::collections::HashMap;
use std::num::ParseIntError;
use std::fmt::Write;

use prettytable::{Table, Row, Cell, Attr, format};
use prettytable::color::GREEN;
use crate::leblanc::rustblanc::hex::Hexadecimal;


pub fn decode_hex(s: &Hexadecimal) -> Result<Vec<u8>, ParseIntError> {
    let s = s.to_string();
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> Hexadecimal {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    Hexadecimal::from_string(s)
}

pub fn encode_hex_u16(bytes: &[u16]) -> Hexadecimal {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    Hexadecimal::from_string(s)
}

pub fn decode_hex_u16(s: &Hexadecimal) -> Result<Vec<u16>, ParseIntError> {
    let s = s.to_string();
    (0..s.len())
        .step_by(2)
        .map(|i| u16::from_str_radix(&s[i..i + 4], 16))
        .collect()
}

#[derive(Clone)]
pub struct Timings {
    pub map: Option<HashMap<String, Timing>>,
}

impl Timings {
    pub fn setup(&mut self) {
        if self.map.is_none() {
            self.map = Some(HashMap::new());
        }
    }

    pub fn lock(&mut self, name: String) {
        let zero_timing = Timing{count: 0, locked_calls: 0, time: 0.0, locks: 0};
        let mut timing = *self.map.as_ref().unwrap().get(&name).unwrap_or(&zero_timing);
        timing.locks += 1;
        self.map.as_mut().unwrap().insert(name, timing);
    }

    pub fn add_timing(&mut self, name: String, duration: f64) {
        let zero_timing = Timing{count: 0, locked_calls: 0, time: 0.0, locks: 0};
        let mut timing = *self.map.as_ref().unwrap().get(&name).unwrap_or(&zero_timing);
        if timing.locks > 0 {
            timing.locks -= 1;
        }
        if timing.locks == 0 {
            timing.count += 1;
            timing.time += duration;
        } else {
            timing.locked_calls += 1;
        }
        self.map.as_mut().unwrap().insert(name, timing);
    }

    pub fn print_timing(&self) {
        let mut table = Table::new();
        table.set_titles(Row::new(vec![
            Cell::new("Instruction")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
            Cell::new("Calls")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
            Cell::new("Locked Calls")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
            Cell::new("Avg Time")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
            Cell::new("Avg Time (with Lock)")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
            Cell::new("Total Time")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(GREEN)),
        ]));

        let mut entry_vec: Vec<(String, Timing)> = self.map.as_ref().unwrap().clone().into_iter().collect();
        entry_vec.sort_by(|a, b| (b.1.time/(b.1.count + b.1.locked_calls) as f64).total_cmp(&(a.1.time / (a.1.count + a.1.locked_calls) as f64)));
        for entry in entry_vec {
            let timing = entry.1;
            table.add_row(Row::new(vec![
                Cell::new(&entry.0),
                Cell::new(&timing.count.to_string()),
                Cell::new(&timing.locked_calls.to_string()),
                Cell::new(&(timing.time / timing.count as f64).to_string()),
                Cell::new(&(timing.time / (timing.count + timing.locked_calls) as f64).to_string()),
                Cell::new(&timing.time.to_string())
            ]));
        }
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.print_tty(true);
    }
}


#[derive(Copy, Clone)]
pub struct Timing {
    pub count: u32,
    pub locked_calls: u32,
    pub time: f64,
    pub locks: u32
}

impl Timing {
    pub fn zero() -> Timing {
        Timing {
            count: 0,
            locked_calls: 0,
            time: 0.0,
            locks: 0
        }
    }

}