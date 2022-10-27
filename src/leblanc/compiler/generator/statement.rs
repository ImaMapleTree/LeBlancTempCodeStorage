use std::mem::take;
use crate::bytes;
use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::CompileInfo;
use crate::leblanc::compiler::generator::converters::stmnt_to_conditional;
use crate::leblanc::compiler::generator::instruction_generator::InstructionGenerator;
use crate::leblanc::compiler::parser::ast::{Conditional, Statement, Stmnt};
use crate::leblanc::core::interpreter::instructions2::Instruction2;
use crate::leblanc::core::interpreter::instructions2::Instruction2::JUMP;


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
            Stmnt::Conditional { conditional: _ } => {},
            Stmnt::MultiConditional { conditionals} => {
                let mut deferred = take(&mut self.instruct_gen);
                let mut other_gens: Vec<InstructionGenerator> = vec![];
                let mut total_instructs = 0;
                for c in stmnt_to_conditional(conditionals).iter().rev() {
                    match &c.data {
                        Conditional::If { condition, statement } => {
                            self.determine_expression(condition)?;
                            let conditional_instruct = self.instruct_gen.remove_last();
                            let future_index = self.instruct_gen.bytecode_lines();
                            self.instruct_gen.refresh();
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                            let mut own_amount = self.instruct_gen.close();
                            if let Instruction2::RETURN(..) = self.instruct_gen.last_instruction() {}
                            else {
                                self.instruct_gen.add_instruction(self.instruct_gen.line() as usize, JUMP(0, bytes![total_instructs]));
                                own_amount += 1;
                            }
                            let instruct = create_conditional_instruct(conditional_instruct, own_amount);
                            if let Some(bytecode) = self.instruct_gen.get_bytecode_mut(future_index) {
                                bytecode.add_instruction2(instruct);
                                self.instruct_gen.bump_count(1);
                            }
                            other_gens.push(take(&mut self.instruct_gen));
                        }
                        Conditional::ElseIf { condition, statement } => {
                            self.instruct_gen.open();
                            self.determine_expression(condition)?;
                            let conditional_instruct = self.instruct_gen.remove_last();
                            let future_index = self.instruct_gen.bytecode_lines();
                            self.instruct_gen.refresh();
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                            let mut own_amount = self.instruct_gen.close();
                            if let Instruction2::RETURN(..) = self.instruct_gen.last_instruction() {}
                            else {
                                self.instruct_gen.add_instruction(self.instruct_gen.line() as usize, JUMP(0, bytes![total_instructs]));
                                own_amount += 1;
                            }
                            let instruct = create_conditional_instruct(conditional_instruct, own_amount);
                            if let Some(bytecode) = self.instruct_gen.get_bytecode_mut(future_index) {
                                bytecode.add_instruction2(instruct);
                                self.instruct_gen.bump_count(1);
                            }
                            let mut this_gen = take(&mut self.instruct_gen);
                            total_instructs += this_gen.close();
                            other_gens.push(this_gen);
                        }
                        Conditional::Else { statement } => {
                            self.instruct_gen.open();
                            self.determine_statement(statement)?;
                            let mut this_gen = take(&mut self.instruct_gen);
                            total_instructs += this_gen.close();
                            other_gens.push(this_gen);
                        }
                    }
                }
                deferred.bump_count(total_instructs);
                self.instruct_gen = deferred;
                for mut gen in other_gens.into_iter().rev() {
                    gen.refresh();
                    self.instruct_gen.add_instruction_bytecode(gen.instructions_mut())
                }
            }
            Stmnt::While { condition, statement } => {
                self.instruct_gen.open();
                self.instruct_gen.open();
                self.determine_expression(condition)?;
                let conditional_instruct = self.instruct_gen.remove_last();
                let future_index = self.instruct_gen.bytecode_lines();

                let expression_length = self.instruct_gen.close();
                self.instruct_gen.refresh();
                self.instruct_gen.open();
                self.determine_statement(statement)?;
                if let Instruction2::RETURN(..) = self.instruct_gen.last_instruction() {}
                else {
                    let close = self.instruct_gen.close();
                    self.instruct_gen.add_instruction(self.instruct_gen.line() as usize, Instruction2::JUMP_BACK(0, bytes![close + expression_length + 1]));
                }
                let total_length = self.instruct_gen.close();
                if let Some(bytecode) = self.instruct_gen.get_bytecode_mut(future_index) {
                    let instruct = create_conditional_instruct(conditional_instruct, total_length - expression_length);
                    bytecode.add_instruction2(instruct);
                    self.instruct_gen.bump_count(1);
                }


            }
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

}

fn create_conditional_instruct(previous: Instruction2, LEI: usize) -> Instruction2 {
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