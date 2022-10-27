use core::fmt::{Display, Formatter};
use std::ffi::OsStr;


static mut STRING_REFS: Vec<String> = Vec::new();

#[derive(Copy, Clone, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct CopyString {
    string: &'static str
}

impl CopyString {
    fn get_string_ref(string: String) -> &'static String {
        unsafe {
            let prev_ref = STRING_REFS.iter().find(|p| *p == &string);
            match prev_ref {
                Some(reference) => reference,
                None => {
                    STRING_REFS.push(string);
                    STRING_REFS.last().unwrap()
                }
            }
        }
    }

    pub fn new<T: Display>(string: T) -> CopyString {
        CopyString {
            string: CopyString::get_string_ref(string.to_string())
        }
    }

    pub const fn constant() -> CopyString {
        CopyString { string: "" }
    }

    pub fn str(&self) -> &str {
        self.string
    }
}

impl From<String> for CopyString {
    fn from(str: String) -> Self {
        CopyString { string: CopyString::get_string_ref(str) }
    }
}

impl From<&String> for CopyString {
    fn from(str: &String) -> Self {
        CopyString { string: CopyString::get_string_ref(str.to_owned()) }
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

impl AsRef<OsStr> for CopyString {
    fn as_ref(&self) -> &OsStr {
        self.string.as_ref()
    }
}