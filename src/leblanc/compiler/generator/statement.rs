use crate::bytes;
use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::CompileInfo;
use crate::leblanc::compiler::generator::converters::stmnt_to_conditional;
use crate::leblanc::compiler::parser::ast::{Conditional, Statement, Stmnt};
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::rustblanc::lazy_store::Lazy;


impl CodeGenerator {
    pub fn determine_statement(&mut self, statement: &Statement) -> Result<CompileInfo, ()> {
        match &statement.data {
            Stmnt::Global { .. } => {}
            Stmnt::Block { statements } => {
                for line in statements {
                    self.level += 1;
                    let result = self.determine_statement(line);
                    if self.reporter.should_exit() { return result; }
                    self.level -= 1;
                }
            }
            Stmnt::Line { expr } => return self.determine_expression(expr),
            Stmnt::Conditional { conditional } => {},
            Stmnt::MultiConditional { conditionals} => {
                for c in stmnt_to_conditional(conditionals).iter().rev() {
                    match &c.data {
                        Conditional::If { condition, statement } => {
                            let other_amount = self.instruct_gen.close_or_zero();
                            self.determine_expression(condition)?;
                            let conditional_instruct = self.instruct_gen.remove_last();
                            let future_index = self.instruct_gen.bytecode_lines();
                            self.instruct_gen.refresh();
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                            let own_amount = self.instruct_gen.close();
                            let instruct = self.create_conditional_instruct(conditional_instruct, other_amount + own_amount);
                            if let Some(bytecode) = self.instruct_gen.get_bytecode_mut(future_index) {
                                bytecode.add_instruction2(instruct)
                            }
                        }
                        Conditional::ElseIf { condition, statement } => {
                            let other_amount = self.instruct_gen.close();
                            self.instruct_gen.open_with_amount(other_amount);
                            self.determine_expression(condition)?;
                            let conditional_instruct = self.instruct_gen.remove_last();
                            let future_index = self.instruct_gen.bytecode_lines();
                            self.instruct_gen.refresh();
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                            let own_amount = self.instruct_gen.close();
                            let instruct = self.create_conditional_instruct(conditional_instruct, other_amount + own_amount);
                            if let Some(bytecode) = self.instruct_gen.get_bytecode_mut(future_index) {
                                bytecode.remove();
                                bytecode.add_instruction2(instruct)
                            }
                        }
                        Conditional::Else { statement } => {
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                        }
                    }
                }
            }
            Stmnt::While { .. } => {}
            Stmnt::For { .. } => {}
            Stmnt::InfLoop { .. } => {}
            Stmnt::Try { .. } => {}
            Stmnt::Except { .. } => {}
            Stmnt::Return { statement } => {
                self.determine_statement(statement)?;
                self.instruct_gen.add_instruction(statement.location.line, Instruction2::RETURN(0, [1]))
            },
        }
        Ok(CompileInfo::default())
    }

    fn create_conditional_instruct(&self, previous: Instruction2, LEI: usize) -> Instruction2 {
        match previous {
            Instruction2::EQUALS(_, _) => Instruction2::IF_EQUALS(0, bytes![LEI]),
            Instruction2::NOT_EQUALS(_, _) => Instruction2::IF_NOT_EQUALS(0, bytes![LEI]),
            Instruction2::GREATER_EQUALS(_, _) => Instruction2::IF_GREATER_EQUALS(0, bytes![LEI]),
            Instruction2::GREATER(_, _) => Instruction2::IF_GREATER(0, bytes![LEI]),
            Instruction2::LESS_EQUALS(_, _) => Instruction2::IF_LESS_EQUALS(0, bytes![LEI]),
            Instruction2::LESS(_, _) => Instruction2::IF_LESS(0, bytes![LEI]),
            _ => Instruction2::NOREF(0, [])
        }
    }

}