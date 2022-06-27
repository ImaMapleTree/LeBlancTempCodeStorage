use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ClassMeta {
    pub name: String,
    pub supertypes: Vec<ClassMeta>,
    pub parse_id: u32,
}

impl ClassMeta {
    pub fn default<'a>(name: String, parse_id: u32) -> ClassMeta {
        ClassMeta {
            name,
            supertypes: vec![],
            parse_id

        }
    }

    pub fn null() -> ClassMeta {
        ClassMeta {
            name: "NULL".to_string(),
            supertypes: vec![],
            parse_id: 0
        }
    }
}

impl Display for ClassMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
