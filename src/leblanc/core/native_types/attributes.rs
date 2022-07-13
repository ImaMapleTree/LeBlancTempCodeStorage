use crate::leblanc::core::native_types::LeBlancType;

pub fn can_add_self(lbt: &LeBlancType) -> bool {
    matches!(lbt, LeBlancType::Short | LeBlancType::Int | LeBlancType::Int64 | LeBlancType::Int128 | LeBlancType::Arch | LeBlancType::String)
}