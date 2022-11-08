use serde::Serialize;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum LBType {
    Boolean,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Dynamic,
    NAT,
    Class(u64),
}

impl LBType {
    pub fn to_bytes(self) -> [u8; 8] {
        let n = match self {
            LBType::Boolean => 0,
            LBType::Char => 1,
            LBType::Short => 2,
            LBType::Int => 3,
            LBType::Long => 4,
            LBType::Float => 5,
            LBType::Double => 6,
            LBType::String => 7,
            LBType::Dynamic => 8,
            LBType::NAT => 9,
            LBType::Class(n2) => 10 + n2
        };
        n.to_le_bytes()
    }
}

impl From<LeBlancType> for LBType {
    fn from(typ: LeBlancType) -> Self {
        match typ {
            LeBlancType::Class(_) => LBType::Class(0),
            LeBlancType::Flex => LBType::Dynamic,
            LeBlancType::SelfType => panic!("Currently Unresolved type: SelfType"),
            LeBlancType::Char => LBType::Char,
            LeBlancType::Short => LBType::Short,
            LeBlancType::Int => LBType::Int,
            LeBlancType::Int64 => LBType::Long,
            LeBlancType::Int128 => LBType::Long,
            LeBlancType::Arch => LBType::Long,
            LeBlancType::Float => LBType::Float,
            LeBlancType::Double => LBType::Double,
            LeBlancType::Boolean => LBType::Boolean,
            LeBlancType::String => LBType::String,
            LeBlancType::Group => panic!("Currently Unresolved type: Group"),
            LeBlancType::Function => panic!("Currently Unresolved type: Function"),
            LeBlancType::Module => panic!("Currently Unresolved type: Module"),
            LeBlancType::Dynamic => LBType::Dynamic,
            LeBlancType::Exception => panic!("Currently Unresolved type: Exception"),
            LeBlancType::Derived(_) => panic!("Currently Unresolved type: Derived"),
            LeBlancType::Promise => panic!("Currently Unresolved type: Promise"),
            LeBlancType::SuperLambda => panic!("Currently Unresolved type: SuperLambda"),
            LeBlancType::ConstantFlex(_) => panic!("Currently Unresolved type: ConstantFlex"),
            LeBlancType::Trait(_, _) => panic!("Currently Unresolved type: Trait"),
            LeBlancType::Marker => panic!("Currently Unresolved type: Marker"),
            LeBlancType::Null => LBType::NAT
        }
    }
}