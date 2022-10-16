use core::fmt::{Display, Formatter};
use strum_macros::EnumVariantNames;
use strum::VariantNames;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};
use strum::{EnumIter, IntoEnumIterator};

use crate::leblanc::compiler::parser::ast::{BinaryOperator, Comparator, UnaryOperator};
use crate::leblanc::core::interpreter::instructions::InstructionBase::*;
use crate::leblanc::rustblanc::hex::Hexadecimal;


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, EnumVariantNames, strum_macros::Display, EnumIter, Copy, Clone, PartialOrd, Hash)]
pub enum InstructionBase {
    Zero,
    NotImplemented,
    Dummy(u32),
    InstructionMarker,
    Jump,

    UnaryPositive,
    UnaryNegative,
    UnaryNot,
    UnaryInverse,
    UnaryIncrement,
    UnaryDecrement,

    InPlaceAdd,
    BinaryAdd,
    BinarySubtract,
    BinaryMultiply,
    BinaryDivide,
    BinaryPower,
    BinaryModulo,
    BinaryLShift,
    BinaryRShift,
    BinaryNot,
    BinaryInverse,
    BinaryAnd,
    BinaryOr,
    BinaryXor,

    Equality(u8),
    ComparatorIf,
    ComparatorElseIf,
    ComparatorElse,

    ForLoop,
    WhileLoop,

    LoadConstant,
    LoadLocal,
    LoadGlobal,
    LoadFunction,
    LoadAttr,
    StoreLocal,
    StoreGlobal,
    StoreUndefined,

    CallClassMethod,
    CallFunction,
    Return,
    Cast,
    AttributeAccess,
    AttributeStore,
    ElementAccess,
    ElementStore,

    IteratorSetup(u16),
    ListSetup,
    MakeSlice,

    Group,

    UseModule,
    MapMatch,

}

impl Hexable for InstructionBase {
    fn to_hex(&self, bytes: usize) -> Hexadecimal {
        let variants: &[&'static str] = InstructionBase::VARIANTS;
        encode_hex(&(variants.iter().position(|s| *s.to_string() == self.to_string()).unwrap() as u32).to_be_bytes()[4-bytes..4])
    }

    fn from_hex(hex: &Hexadecimal) -> Self {
        let mut bytes = decode_hex(hex).unwrap();
        while bytes.len() < 4 { bytes.insert(0, 0) };
        let instruct_number = u32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap());
        InstructionBase::iter().enumerate().find(|&(i, _)| i == instruct_number as usize).unwrap().1
    }
}

impl InstructionBase {
    pub fn instruct(&self, arg: u16, line_number: u32) -> Instruction {
        Instruction::new(*self, arg, line_number)
    }

    pub fn to_value(&self) -> u32 {
        let variants: &[&'static str] = InstructionBase::VARIANTS;
        variants.iter().position(|s| *s.to_string() == self.to_string()).unwrap() as u32
    }

    pub fn to_instruction(self, arg: u16, line_number: u32) -> Instruction {
        Instruction::new(self, arg, line_number)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Instruction {
    pub instruct: InstructionBase,
    pub arg: u16,
    pub line_number: u32
}

impl Instruction {
    pub fn new(instruct: InstructionBase, arg: u16, line_number: u32) -> Instruction {
        Instruction {
            instruct,
            arg,
            line_number
        }
    }

    pub fn empty() -> Instruction {
        Instruction {
            instruct: Zero,
            arg: 0,
            line_number: 0
        }
    }

    pub fn base(&self) -> InstructionBase {
        self.instruct
    }
}

pub fn comparator_instruct(comparator: Comparator) -> u16 {
    match comparator {
        Comparator::Equal => 0,
        Comparator::NotEqual => 1,
        Comparator::GreaterEqual => 4,
        Comparator::LesserEqual => 5,
        Comparator::Greater => 2,
        Comparator::Lesser => 3,
        Comparator::In => 6,
    }
}

pub fn binary_instruct(binary_op: BinaryOperator) -> InstructionBase {
    match binary_op {
        BinaryOperator::BinAdd => BinaryAdd,
        BinaryOperator::BinSub => BinarySubtract,
        BinaryOperator::BinMul => BinaryMultiply,
        BinaryOperator::BinDiv => BinaryDivide,
        BinaryOperator::BinPow => BinaryPower,
        BinaryOperator::BinMod => BinaryModulo,
        BinaryOperator::BinLShift => BinaryLShift,
        BinaryOperator::BinRShift => BinaryRShift
    }
}

pub fn unary_instruct(unary_op: UnaryOperator) -> InstructionBase {
    match unary_op {
        UnaryOperator::UPositive => UnaryPositive,
        UnaryOperator::UNegative => UnaryNegative,
        UnaryOperator::UNot => UnaryNot,
        UnaryOperator::UInverse => UnaryInverse,
        UnaryOperator::UIncrement => UnaryIncrement,
        UnaryOperator::UDecrement => UnaryDecrement,
    }
}