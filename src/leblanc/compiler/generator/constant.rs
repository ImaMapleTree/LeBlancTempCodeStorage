use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::{ConstInfo};
use crate::leblanc::compiler::parser::ast::{Const};
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::rustblanc::lazy_store::Strategy;

impl CodeGenerator {
    pub fn determine_constant(&mut self, constant: &Const) -> Result<ConstInfo, ()>{
        let ty = match &constant {
            Const::String(_, _) => {
                LeBlancType::String
            }
            Const::Whole(_, ty, _) => {
                match ty {
                    None => LeBlancType::Int,
                    Some(tyy) => *tyy
                }
            }
            Const::Float(_, ty, _) => {
                match ty {
                    None => LeBlancType::Float,
                    Some(tyy) => *tyy
                }
            }
            Const::Boolean(_, _) => {
                LeBlancType::Boolean
            }
        };
        Ok(ConstInfo::new(self.get_constant_id(constant.clone()), ty))
    }

    fn get_constant_id(&mut self, c: Const) -> usize {
        self.constant_register.get_or_add(c, Strategy::STANDARD).0
    }
}