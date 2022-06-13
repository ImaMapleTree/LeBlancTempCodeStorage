use std::ops::Index;
use std::str::FromStr;
use strum_macros::EnumVariantNames;
use strum::VariantNames;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::utils::{decode_hex, encode_hex};
use strum::{EnumIter, IntoEnumIterator};
use crate::{CompileVocab, TypedToken};
use crate::CompileVocab::KEYWORD;
use crate::leblanc::compiler::lang::leblanc_keywords::LBKeyword;
use crate::leblanc::compiler::lang::leblanc_operators::LBOperator;
use crate::leblanc::core::interpreter::instructions::Instruction::*;


#[derive(Debug, PartialEq, EnumVariantNames, strum_macros::Display, EnumIter)]
pub enum Instruction {
    Zero,
    NotImplemented,
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
    Comparator(u8),

    For,
    While,

    LoadConstant,
    LoadLocal,
    LoadGlobal,
    StoreLocal,
    StoreGlobal,
    StoreUndefined,

    CallFunction,
    Return,
    Cast,
    AttributeAccess,
    AttributeStore,

    UseModule,
    MapMatch,

}

impl Hexable for Instruction {
    fn to_hex(&self, bytes: Option<u32>) -> String {
        let byte_amount = bytes.unwrap_or(4) as usize;
        let variants: &[&'static str] = Instruction::VARIANTS;
        encode_hex(&(variants.iter().position(|s| s.to_string() == self.to_string()).unwrap() as u32).to_be_bytes()[4-byte_amount..4])
    }

    fn from_hex(string: String) -> Self {
        let mut bytes = decode_hex(&string).unwrap();
        while bytes.len() < 4 { bytes.insert(0, 0) };
        let instruct_number = u32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap());
        return Instruction::iter().enumerate().filter(|&(i, _)| i == instruct_number as usize).next().unwrap().1;
    }
}

impl Instruction {
    pub fn from_compile_vocab(token: TypedToken) -> Instruction {
        return match token.lang_type() {
            CompileVocab::CONSTANT(_) => LoadConstant,
            CompileVocab::VARIABLE(_) => {
                if token.global() { LoadGlobal }
                else { LoadLocal }
            },
            CompileVocab::FUNCTION => CallFunction,
            CompileVocab::OPERATOR(op) => {
                match op {
                    LBOperator::Plus => BinaryAdd,
                    LBOperator::Minus => BinarySubtract,
                    LBOperator::Multiply => BinaryMultiply,
                    LBOperator::Divide => BinaryDivide,
                    LBOperator::Power => BinaryPower,
                    LBOperator::Modulo => BinaryModulo,
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
                    LBOperator::Cast => Cast,
                    LBOperator::Attribute => AttributeAccess,
                    LBOperator::NULL => Zero,
                }
            }
            CompileVocab::SPECIAL(_) => Zero,
            CompileVocab::KEYWORD(keyword) => {
                match keyword {
                    LBKeyword::Using => UseModule,
                    LBKeyword::Return => Return,
                    LBKeyword::For => For,
                    LBKeyword::While => While,
                    LBKeyword::If => Comparator(0),
                    LBKeyword::ElseIf => Comparator(1),
                    LBKeyword::Else => Comparator(2),
                    LBKeyword::Class => NotImplemented,
                    _ => Zero
                }
            }
            CompileVocab::MODULE(_) => {}
            CompileVocab::BOUNDARY(_) => {}
            CompileVocab::CONSTRUCTOR(_) => {}
            CompileVocab::EXTENSION(_) => {}
            CompileVocab::CLASS(_) => {}
            CompileVocab::TYPE(_) => {}
            CompileVocab::UNKNOWN(_) => {}
        }
    }
}