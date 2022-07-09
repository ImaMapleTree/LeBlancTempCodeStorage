use core::fmt::{Display, Formatter};
use std::ops::Add;

static mut STRING_REFS: Vec<String> = Vec::new();

#[derive(Copy, Clone, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct CopyString {
    string: &'static str
}

impl CopyString {
    pub fn new<T: Display>(string: T) -> CopyString {
        let string = string.to_string();
        let string_ref = unsafe {
            let prev_ref = STRING_REFS.iter().find(|p| *p == &string);
            match prev_ref {
                Some(reference) => reference,
                None => {
                    STRING_REFS.push(string);
                    STRING_REFS.last().unwrap()
                }
            }
        };

        CopyString {
            string: string_ref
        }
    }
}

impl Default for CopyString {
    fn default() -> Self {
        CopyString {
            string: ""
        }
    }
}

impl Display for CopyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.string)
    }
}

pub trait CopyStringable {
    fn to_cstring(self) -> CopyString;
}

impl<T: Display> CopyStringable for T {
    fn to_cstring(self) -> CopyString {
        CopyString::new(self)
    }
}

impl<T: Display> PartialEq<T> for CopyString {
    fn eq(&self, other: &T) -> bool {
        self.string == other.to_string()
    }
}