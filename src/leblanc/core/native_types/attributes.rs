use crate::LeBlancType;

pub fn can_add_self(lbt: &LeBlancType) -> bool {
    return match lbt {
        LeBlancType::Short => true,
        LeBlancType::Int => true,
        LeBlancType::Int64 => true,
        LeBlancType::Int128 => true,
        LeBlancType::Arch => true,
        LeBlancType::Float => true,
        LeBlancType::Double => true,
        LeBlancType::String => true,
        _ => false
    }
}