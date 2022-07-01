use strum_macros::EnumVariantNames;
use strum::VariantNames;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};
use strum::{EnumIter, IntoEnumIterator};
use crate::{CompileVocab, TypedToken};
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_lang::{BoundaryType, FunctionType, Specials};
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::core::interpreter::instructions::InstructionBase::*;
use crate::leblanc::rustblanc::hex::Hexadecimal;


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, EnumVariantNames, strum_macros::Display, EnumIter, Copy, Clone, PartialOrd, Hash)]
pub enum InstructionBase {
    Zero,
    NotImplemented,
    Dummy(u32),
    InstructionMarker,
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
    Comparator_If,
    Comparator_ElseIf,
    Comparator_Else,

    ForLoop,
    WhileLoop,

    LoadConstant,
    LoadLocal,
    LoadGlobal,
    LoadFunction,
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

    IteratorSetup(u16),
    ListSetup,
    MakeSlice,


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
    pub fn to_value(&self) -> u32 {
        let variants: &[&'static str] = InstructionBase::VARIANTS;
        variants.iter().position(|s| *s.to_string() == self.to_string()).unwrap() as u32
    }

    pub fn from_compile_vocab(token: &TypedToken) -> InstructionBase {
        match token.lang_type() {
            CompileVocab::CONSTANT(_) => LoadConstant,
            CompileVocab::VARIABLE(_) => {
                if token.global() { LoadGlobal }
                else { LoadLocal }
            },
            CompileVocab::FUNCTION(ft) => {
                if ft == FunctionType::Header {
                    Zero
                } else if ft == FunctionType::Reference {
                    LoadFunction
                } else {
                    CallFunction
                }
            },
            CompileVocab::OPERATOR(op) => {
                match op {
                    LBOperator::Plus => BinaryAdd,
                    LBOperator::Minus => BinarySubtract,
                    LBOperator::Multiply => BinaryMultiply,
                    LBOperator::Divide => BinaryDivide,
                    LBOperator::Power => BinaryPower,
                    LBOperator::Modulo => BinaryModulo,
                    LBOperator::Or => BinaryOr,
                    LBOperator::And => BinaryAnd,
                    LBOperator::Not => BinaryNot,
                    LBOperator::Assign => StoreUndefined,
                    LBOperator::Inverse => BinaryInverse,
                    LBOperator::Equals => Equality(0),
                    LBOperator::NotEquals => Equality(1),
                    LBOperator::GreaterThan => Equality(2),
                    LBOperator::LessThan => Equality(3),
                    LBOperator::GreaterThanOrEqual => Equality(4),
                    LBOperator::LessThanOrEqual => Equality(5),
                    LBOperator::LShift => BinaryLShift,
                    LBOperator::RShift => BinaryRShift,
                    LBOperator::Match => MapMatch,
                    LBOperator::Increment => Dummy(1),
                    LBOperator::Cast => Cast,
                    LBOperator::QuickList => IteratorSetup(0),
                    LBOperator::Slice => MakeSlice,
                    LBOperator::Index => ElementAccess,
                    LBOperator::AssignEach => Zero,
                    LBOperator::NULL => Zero,
                }
            }
            CompileVocab::KEYWORD(keyword) => {
                match keyword {
                    LBKeyword::Using => UseModule,
                    LBKeyword::Return => Return,
                    LBKeyword::For => ForLoop,
                    LBKeyword::While => WhileLoop,
                    LBKeyword::If => Comparator_If,
                    LBKeyword::ElseIf => Comparator_ElseIf,
                    LBKeyword::Else => Comparator_Else,
                    LBKeyword::Class => NotImplemented,
                    _ => Zero
                }
            }
            CompileVocab::SPECIAL(special, val) => {
                match special {
                    Specials::RangeMarker => IteratorSetup(val),
                    _ => Zero
                }
            },
            CompileVocab::CONSTRUCTOR(_) => NotImplemented,
            CompileVocab::CLASS(_) => NotImplemented,
            CompileVocab::BOUNDARY(bound) => {
                match bound {
                    BoundaryType::BracketClosed => ListSetup,
                    BoundaryType::BracketOpen => InstructionMarker,
                    _ => Zero
                }
            }
            _ => Zero
        }
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