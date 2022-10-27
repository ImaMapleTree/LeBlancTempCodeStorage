use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::{ConditionalInfo};
use crate::leblanc::compiler::parser::ast::Conditional;

impl CodeGenerator {
    pub fn determine_conditional(&mut self, conditional: &Conditional) -> Result<ConditionalInfo, ()> {
        match conditional {
            Conditional::If { condition, statement } => {
                self.determine_expression(condition);
                self.determine_statement(statement)?;
                Ok(ConditionalInfo { condition: 0 })
            }
            Conditional::ElseIf { condition, statement } => {
                self.determine_expression(condition);
                self.determine_statement(statement)?;
                Ok(ConditionalInfo { condition: 1 })
            }
            Conditional::Else { statement } => {
                self.determine_statement(statement)?;
                Ok(ConditionalInfo { condition: 2 })
            }
        }
    }
}