use crate::bytes;
use crate::leblanc::compiler::generator::CodeGenerator;
use crate::leblanc::compiler::generator::context::CompileInfo;
use crate::leblanc::compiler::generator::generator_types::FunctionSignature;
use crate::leblanc::compiler::parser::ast::{BinaryOperator, Comparator, Expr, Expression, Ident};
use crate::leblanc::compiler::parser::ast_structs::TypedVariable;
use crate::leblanc::core::internal::methods::builtins::BUILTIN_METHODS;

use crate::leblanc::core::interpreter::instructions2::Instruction2::*;


use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::core::native_types::LeBlancType;

use crate::leblanc::rustblanc::lazy_store::Strategy;
use crate::leblanc::rustblanc::unsafe_vec::UnsafeVec;


impl CodeGenerator {
    pub fn determine_expression(&mut self, expression: &Expression) -> Result<CompileInfo, ()> {
        match &expression.data {
            Expr::Break => {}
            Expr::RangeExpression { start, bound, step } => {
                return self.determine_expression(start)
                    .and(self.determine_expression(bound))
                    .and(self.determine_expression(step));
            }
            Expr::StaticMethodCall { method_name, args } => {
                let location = method_name.location;
                // FUTURE
                //  Eventually with the implementation of kwargs, we're going to do a match
                //  on the kwarg expression and have pre-logic if it matches that
                //  otherwise we evaluate as normal

                let mut arguments: Vec<LeBlancArgument> = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    let result = self.determine_expression(arg)?;
                    arguments.push(LeBlancArgument::default(result.get_type(), i as u32));
                }

                let info = self.resolve_ident_expr(method_name, false)?;
                let ident = info.get_ident();
                if ident.is_none() {
                    return Err(());
                }
                let name = ident.unwrap();
                let argument_count = arguments.len();


                let signature = FunctionSignature::new(&name.resolve(), arguments, vec![], location);
                let function_index = self.func_map.index(&signature, Strategy::STANDARD);
                match function_index {
                    None => {
                        println!("Func map: {:#?}", self.func_map);
                        println!("Signature: {:#?}", signature);
                        eprintln!("ERROR FUNCTION NOT FOUND: {}", name.resolve())
                    },
                    Some(index) => {
                        let instruct = if index >= BUILTIN_METHODS as usize {
                            CALL_NORMAL(0, bytes![index - BUILTIN_METHODS as usize + 1, argument_count])
                        } else {
                            CALL_BUILTIN(0, bytes![index, argument_count])
                        };
                        self.instruct_gen.add_instruction(location.line, instruct);
                        let real_func = self.func_map.get(index).unwrap();
                        return Ok(CompileInfo::of_type(*real_func.returns().get(0).unwrap()));
                    }
                }
                // method loaded from ident calls
                // we get back the ident of the function as a string, not sure what that's going to look like yet
                // or maybe we just compute a list of matching functions based on the ident

                return Ok(CompileInfo::of_type(LeBlancType::Null))
            }
            Expr::ListIndex { list, slice } => {
                return self.determine_expression(slice)
                    .and(self.determine_expression(list));
            }
            Expr::Slice {left, right } => {
                return self.determine_expression(left)
                    .and(self.determine_expression(right));
            }
            Expr::SeriesIndex { indices } => {
                if let Err(err) = self.evaluate_expressions(indices) { return err; }
                return Ok(CompileInfo::of_type(LeBlancType::Derived(DerivedType::Slice)))
            }
            Expr::Equality { left, comparator, right } => {
                let line = right.location.line;
                self.determine_expression(left)?;
                self.determine_expression(right)?;
               let instruct = match comparator {
                    Comparator::Equal => EQUALS,
                    Comparator::NotEqual => NOT_EQUALS,
                    Comparator::GreaterEqual => GREATER_EQUALS,
                    Comparator::LesserEqual => LESS_EQUALS,
                    Comparator::Greater => GREATER,
                    Comparator::Lesser => LESS,
                    Comparator::In => NOT_EQUALS,
                };
                self.instruct_gen.add_instruction(line, instruct(0, []));
            }
            Expr::List { items } => {
                if let Err(err) = self.evaluate_expressions(items) { return err; }
                return Ok(CompileInfo::of_type(LeBlancType::Derived(DerivedType::List)))
            }
            Expr::ArithPlusMinusOperation { left, op, right } => {
                let line = right.location.line;
                let t = self.determine_expression(left)?;
                self.determine_expression(right)?;
                match op {
                    BinaryOperator::BinAdd => self.instruct_gen.add_instruction(line, BADD_NATIVE(0, [])),
                    BinaryOperator::BinSub => self.instruct_gen.add_instruction(line, BSUB_NATIVE(0, [])),
                    _ => {}
                }
                return Ok(t)
            }
            Expr::ArithMulDivModOperation { left: _, op: _, right: _ } => {}
            Expr::ExponentialOperation { left: _, op: _, right: _ } => {}
            Expr::UnaryOperation { term: _, op: _ } => {}
            Expr::IncrementDecrementOperation { term: _, op: _, postfix: _ } => {}
            Expr::ListAssignment { list: _, expr: _ } => {}
            Expr::TypedAssignment { idents, expr } => {
                let mut evaluated: Vec<Ident> = Vec::new();
                for ident in idents {
                    let typed = TypedVariable::from(ident);
                    let info  = self.add_type(typed.variable, typed.typing)?;
                    match info.get_ident() {
                        None => {
                            println!("ERROR: {:#?}", info);
                            println!("ERROR: {:#?}", self.type_map);
                        }
                        Some(id) => evaluated.push(id.clone()),
                    }
                }
                if let Some(expression) = expr {
                    let result = self.determine_expression(expression)?;
                    for ident in evaluated {
                        let line = ident.location.line;
                        let info = self.validate_type(ident, expression.location, result.get_type(), true)?;
                        let instruct = STORE_VARIABLE(0, bytes![info.id]);
                        self.instruct_gen.add_instruction(line, instruct)
                    }
                }
            }
            Expr::NormalAssignment { idents, expr } => {
                let result = self.determine_expression(expr)?;
                for ident in idents.iter().cloned() {
                    let location = ident.location;
                    let info = self.validate_type(ident, expr.location,result.get_type(), true)?;
                    let instruct = STORE_VARIABLE(0, bytes![info.id]);
                    self.instruct_gen.add_instruction(location.line, instruct);
                }
            }
            Expr::GroupAssignment { assignee: _, group: _ } => {}
            Expr::BlockLambda { variables: _, block: _ } => {}
            Expr::ExprLambda { variables: _, expr: _ } => {}
            Expr::ExceptCatch { errors: _, variable: _ } => {}
            Expr::TypedVariable { typing, variable } => {
                let info = self.add_type(variable.clone(), *typing)?;
                let instruct = LOAD_VARIABLE(0, bytes![info.id]);
                self.instruct_gen.add_instruction(variable.location.line, instruct);
                return Ok(info);
            }
            Expr::Ident { ident } => {
                let info = self.get_type(ident)?;
                let instruct = if info.get_type() == LeBlancType::Function {
                    LOAD_FUNCTION
                } else { LOAD_VARIABLE };
                let instruct = instruct(0, bytes![info.id]);
                self.instruct_gen.add_instruction(ident.location.line, instruct);
                return Ok(info);
            }
            Expr::Constant { constant } => {
                let res = self.determine_constant(constant)?;
                let instruct = LOAD_CONSTANT(0, bytes![res.id]);
                self.instruct_gen.add_instruction(constant.location().line, instruct);
                return Ok(CompileInfo::of_type(res.ty))
            }
        }
        Ok(CompileInfo::of_type(LeBlancType::Null))
    }

    fn evaluate_expressions(&mut self, expressions: &Vec<Expression>) -> Result<(), Result<CompileInfo, ()>> {
        for expr in expressions {
            let result = self.determine_expression(expr);
            if self.reporter.should_exit() { return Err(result); }
        }
        Ok(())
    }

    fn resolve_ident_expr(&self, expression: &Expression, error_on_undef: bool) -> Result<CompileInfo, ()> {
        match &expression.data {
            Expr::Ident { ident } => {
                match self.get_type( ident){
                    Ok(info) => Ok(info),
                    Err(_) => {
                        if error_on_undef {
                            Err(())
                        } else {
                            Ok(CompileInfo::ident(ident.clone(), usize::MAX))
                        }
                    }
                }
            }
            _ => {Err(())}
        }
    }
}