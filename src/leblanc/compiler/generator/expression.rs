use crate::leblanc::compiler::generator::bytecode_generator::BytecodeGenerator;
use crate::leblanc::compiler::generator::generator_types::{GeneratedClass, GeneratedFuncHeader};
use crate::leblanc::compiler::parser::ast::{Expr, Expression};
use crate::leblanc::rustblanc::component_map::ComponentMap;


impl BytecodeGenerator {
    pub fn determine_expression(&mut self, expression: &Expression) {
        match &expression.data {
            Expr::Break => {}
            Expr::RangeExpression { start, bound, step } => {}
            Expr::StaticMethodCall { .. } => {}
            Expr::ClassMethodCall { .. } => {}
            Expr::ListIndex { .. } => {}
            Expr::Slice { .. } => {}
            Expr::SeriesIndex { .. } => {}
            Expr::Equality { .. } => {}
            Expr::List { .. } => {}
            Expr::ArithPlusMinusOperation { .. } => {}
            Expr::ArithMulDivModOperation { .. } => {}
            Expr::ExponentialOperation { .. } => {}
            Expr::UnaryOperation { .. } => {}
            Expr::IncrementDecrementOperation { .. } => {}
            Expr::ListAssignment { .. } => {}
            Expr::TypedAssignment { .. } => {}
            Expr::NormalAssignment { .. } => {}
            Expr::GroupAssignment { .. } => {}
            Expr::BlockLambda { .. } => {}
            Expr::ExprLambda { .. } => {}
            Expr::ExceptCatch { .. } => {}
            Expr::TypedVariable { .. } => {}
            Expr::Ident { .. } => {}
            Expr::Constant { .. } => {}
        }
    }
}