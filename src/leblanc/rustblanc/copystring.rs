use core::fmt::{Display, Formatter};

static mut STRING_REFS: Vec<String> = Vec::new();

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct CopyString {
    string: &'static str
}

impl CopyString {
    pub fn new(string: String) -> CopyString {
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