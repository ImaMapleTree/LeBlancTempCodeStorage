use alloc::rc::Rc;
use alloc::vec::IntoIter;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::iter::{FilterMap};
use crate::leblanc::compiler::bytecode::file_body::FileBodyBytecode;
use crate::leblanc::compiler::bytecode::file_header::FileHeaderBytecode;
use crate::leblanc::compiler::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::compiler::bytecode::instruction_line_bytes::InstructionBytecode;
use crate::leblanc::compiler::bytecode::{LeblancBytecode, ToBytecode};
use crate::leblanc::compiler::compile_types::partial_function::PartialFunction;
use crate::leblanc::compiler::parser::ast::{Cmpnt, Component, Conditional, Const, Expr, Expression, Id, Ident, Statement, Stmnt};
use crate::leblanc::compiler::parser::import_manager::{CompiledImport, import};
use crate::leblanc::compiler::parser::parse_structs::{IdentStore, ScopeSet, ScopeTrack, ScopeValue, SyntaxError};
use crate::leblanc::core::internal::methods::builtins::create_partial_functions;
use crate::leblanc::core::interpreter::instructions::{binary_instruct, comparator_instruct, Instruction, InstructionBase as Instruct, InstructionBase, unary_instruct};
use crate::leblanc::core::interpreter::instructions::InstructionBase::{ListSetup, LoadFunction, LoadLocal};
use crate::leblanc::core::leblanc_argument::LeBlancArgument;
use crate::leblanc::core::native_types::derived::DerivedType;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::core::native_types::LeBlancType;

static mut CONSTANT_TRACK: Option<ConstantTrack> = None;
static mut CURRENT_MODULE: String = String::new();

fn ident_and_set_to_partial(store: &IdentStore, value: &ScopeSet) -> Option<PartialFunction> {
    if let IdentStore::Function(name, args) = store {
        let mut returns = value.iter().next().unwrap().types.clone();
        if returns.is_empty() { returns.push(LeBlancType::Null) }
        Some(PartialFunction {
            name: name.clone(),
            args: LeBlancArgument::from_positional(args),
            returns
        })
    } else {
        None
    }
}

pub fn generate_bytecode(modules: Vec<CompiledImport>, mut type_map: HashMap<String, HashMap<IdentStore, ScopeSet>>) {
    let mut partial_functions = create_partial_functions();

    let mut functions = type_map.iter().map(|(_key, value)| {
        value.iter().filter(|(key, _value)| {
            matches!(key, IdentStore::Function(_name, _types))
        }).collect::<Vec<(&IdentStore, &ScopeSet)>>()
    }).flatten().collect::<Vec<(&IdentStore, &ScopeSet)>>();

    functions.sort_by(|(_key, value), (_key2, value2)| value.get_first_id().unwrap().cmp(&value2.get_first_id().unwrap()));
    functions.iter().for_each(|(key, value)| partial_functions.push(ident_and_set_to_partial(key, value).unwrap()));


    let mut body = FileBodyBytecode::new();
    for module in modules {
        unsafe { CURRENT_MODULE = module.name; }
        let scope = Rc::new(RefCell::new(ScopeTrack::default()));
        function_pass(&mut body, module.components.into_iter().filter_map(|c| {
            if let Cmpnt::Function { header: h, body: b, tags} = c.data {
                Some((h.data, b.data, tags))
            } else {
                None
            }
        }), scope, &mut type_map, &mut partial_functions);
    }



    let mut header = FileHeaderBytecode::new();
    header.set_file_name(&String::from("test.lb"));
    let mut bytecode = LeblancBytecode::new(header, body);
    let file = File::options().truncate(true).write(true).create(true).open("test.lbbc");
    let generated = bytecode.generate();
    file.unwrap().write_all(&hex::decode(generated.to_string()).unwrap()).unwrap();
}



fn function_pass(body: &mut FileBodyBytecode, components: FilterMap<IntoIter<Component>, fn(Component) -> Option<(Cmpnt, Stmnt, Vec<String>)>>, scope_value: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction> ) {
    components.for_each(|tuple| {
        let mut bytecode = generate_function(tuple.1, scope_value.borrow_mut().bump().rc(), type_map, partial_functions);
        add_header_info(&mut bytecode, tuple.0);
        body.add_function(bytecode);
    });
}

fn add_header_info(bytecode: &mut FunctionBytecode, header: Cmpnt) {
    if let Cmpnt::FunctionHeader { name, args, returns } = header {
        bytecode.set_name(name);
        for arg in args {
            if let Expr::TypedVariable {typing, variable: _variable } = arg.data {
                bytecode.add_argument(typing);
            }
        }
    }
}

fn generate_function(body: Stmnt, scope_value: Rc<RefCell<ScopeTrack>>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> FunctionBytecode {
    let mut instructions = vec![];
    unsafe { if CONSTANT_TRACK.is_some() {
        CONSTANT_TRACK.as_mut().unwrap().constants.clear()
    }}
    instruct_stack_statement(&body, scope_value.clone(), &mut instructions, type_map, partial_functions);
    let variables_in_scope = unsafe {type_map.get_mut(&CURRENT_MODULE)}.unwrap().iter().filter_map(|(store, value)| {
        if value.iter().any(|v| v.scope == scope_value.borrow().get_scope_type()) {
            Some((store.clone(), value.get_matching_scope_value(scope_value.borrow().get_scope_type())))
        } else { None }
    }).collect::<Vec<(IdentStore, ScopeValue)>>();


    let mut function_bytecode = FunctionBytecode::default();
    for variable in variables_in_scope {
        if let IdentStore::Variable(id) = variable.0 {
            function_bytecode.add_variable(id, variable.1.id as u32)
        }
    }

    for constant in sorted_vec() {
        println!("CONSTANT: {:#?}", constant);
        match constant {
            Const::String(str) => function_bytecode.add_constant(str.to_hex(0), LeBlancType::String.enum_id() as u16),
            Const::Boolean(truth) => function_bytecode.add_constant(truth.to_hex(128), LeBlancType::Boolean.enum_id() as u16),
            Const::Whole(val, t) => {
                match t {
                    None => function_bytecode.add_constant((val as i32).to_hex(128), LeBlancType::Int.enum_id() as u16),
                    Some(ty) => function_bytecode.add_constant(ty.transform(val.to_string()), ty.enum_id() as u16),
                }
            }
            Const::Float(val, t) => {
                match t {
                    None => function_bytecode.add_constant((val as f64).to_hex(128), LeBlancType::Double.enum_id() as u16),
                    Some(ty) => function_bytecode.add_constant(ty.transform(val.to_string()), ty.enum_id() as u16),
                }
            }
        }
    }

    let mut instruct_line = InstructionBytecode::default();
    let line_number = 0;
    let mut first = true;
    for instruct in instructions {
        println!("Instruct: {:?}", instruct);
        if instruct.line_number != line_number {
            if !first {
                function_bytecode.add_instruction_line(instruct_line.generate())
            }
            instruct_line = InstructionBytecode::default();
            instruct_line.set_line_number(instruct.line_number);
            first = false;
        }
        instruct_line.add_instruction(instruct.instruct.to_hex(2), instruct.arg.to_hex(2));
    }
    if !instruct_line.clone().to_instructions().is_empty() {
        function_bytecode.add_instruction_line(instruct_line.generate())
    }

    function_bytecode
}

fn instruct_stack_statement(statement: &Stmnt, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> StackCount {
    match statement {
        Stmnt::Global { statement } => instruct_stack_statement(&statement.data, ScopeTrack::default().rc(), stack, type_map, partial_functions),
        Stmnt::Block { statements } => {
            let mut statements = statements.clone(); //IMPORTANT I can't think of a better way to do this right now
            statements.reverse();
            let mut stack_count = StackCount::default();
            while !statements.is_empty() {
                let statement = statements.pop().unwrap();
                println!("Next statement: {:?}", statement);
                if let Stmnt::Conditional { conditional } = statement.data {
                    let mut conditional_statements = vec![];
                    let mut next_conditional = Some(conditional);
                    while next_conditional.is_some() {
                        conditional_statements.push(next_conditional.unwrap());
                        let next_statement = statements.pop();
                        if next_statement.is_some() {
                            let unwrapped = next_statement.unwrap();
                            next_conditional = if let Stmnt::Conditional { conditional } = unwrapped.data {
                                Some(conditional)
                            } else { statements.push(unwrapped); None }
                        } else { next_conditional = None }
                    }
                    let mut temp_scopes = vec![];
                    for conditional in conditional_statements {
                        let mut temp_stack = vec![];
                        let (temp_stack_count, eq_stack, cond_stack) = instruct_stack_cond_special(&conditional, scope_value.clone(), &mut temp_stack, type_map, partial_functions);
                        temp_scopes.push((temp_stack_count, eq_stack, cond_stack));
                    }
                    let mut count = 0;
                    for (times, (stack_count,  eq_stack, cond_stack)) in temp_scopes.iter_mut().rev().enumerate() {
                        let mut last_instruct = eq_stack.pop().unwrap();
                        if times > 0 && !cond_stack.is_empty(){
                            stack_count.stack_count += 1;
                            stack_count.instruct_count += 1;
                            cond_stack.push(Instruct::Jump.instruct(count as u16, cond_stack.last().unwrap().line_number));
                            last_instruct.arg += 1;
                        }
                        count += stack_count.instruct_count as u16;
                        eq_stack.push(last_instruct);
                        eq_stack.append(cond_stack);

                    }
                    temp_scopes.iter_mut().for_each(|(stk_count, eq_stack, cond_stack)| {
                        stack_count.add(stk_count);
                        stack.append(eq_stack);
                        stack.append(cond_stack);
                    });
                } else {
                    stack_count.add(&instruct_stack_statement(&statement.data, scope_value.clone(), stack, type_map, partial_functions))
                }
            }
            stack_count
        }
        Stmnt::Line { expr } => { instruct_stack_expr(expr, scope_value, stack, type_map, partial_functions) }
        Stmnt::Conditional { conditional } => {
            instruct_stack_cond(conditional, scope_value, stack, type_map, partial_functions)
        }
        Stmnt::While { condition: condition, statement } => {
            StackCount::default()
        }
        Stmnt::For { variable: variable, iterable: iter, statement } => {
            StackCount::default()
        }
        Stmnt::InfLoop { statement } => {
            StackCount::default()
        }
        Stmnt::Try { statement } => {
            StackCount::default()
        }
        Stmnt::Except { catch, statement } => {
            StackCount::default()
        }
        Stmnt::Return { statement } => {
            instruct_stack_statement(&statement.data, scope_value, stack, type_map, partial_functions)
        }
    }
}

fn instruct_stack_cond(cond: &Conditional, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> StackCount {
    match cond {
        Conditional::If { condition, statement } => {
            let mut stack_count = instruct_stack_expr(condition, scope_value.clone(), stack, type_map, partial_functions);
            let mut temp_stack = vec![];
            let other_count = instruct_stack_statement(&statement.data, scope_value, &mut temp_stack, type_map, partial_functions);
            let instructs = other_count.instruct_count;
            println!("Instructs: {}", instructs);
            for instruct in &temp_stack {
                println!("Instruct: {:?}", instruct);
            }
            stack_count.add(&other_count);
            stack_count = stack_count.macro_instruct(stack, Instruct::Comparator_If.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            stack.append(&mut temp_stack);
            stack_count
        }
        Conditional::ElseIf { condition, statement } => {
            let mut stack_count = instruct_stack_expr(condition, scope_value.clone(), stack, type_map, partial_functions);
            let mut temp_stack = vec![];
            let other_count = instruct_stack_statement(&statement.data, scope_value, &mut temp_stack, type_map, partial_functions);
            let instructs = other_count.instruct_count;
            stack_count.add(&other_count);
            stack_count = stack_count.macro_instruct(stack, Instruct::Comparator_ElseIf.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            stack.append(&mut temp_stack);
            stack_count

        }
        Conditional::Else { statement } => {
            let mut temp_stack = vec![];
            let mut stack_count = instruct_stack_statement(&statement.data, scope_value, &mut temp_stack, type_map, partial_functions);
            let instructs = stack_count.instruct_count;
            stack_count = stack_count.macro_instruct(stack, Instruct::Comparator_Else.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            stack.append(&mut temp_stack);
            stack_count

        }
    }
}

fn instruct_stack_cond_special(cond: &Conditional, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> (StackCount, Vec<Instruction>, Vec<Instruction>) {
    match cond {
        Conditional::If { condition, statement } => {
            let mut other_stack = vec![];
            let mut stack_count = instruct_stack_expr(condition, scope_value.clone(), &mut other_stack, type_map, partial_functions);
            let mut temp_stack = vec![];
            let other_count = instruct_stack_statement(&statement.data, scope_value.borrow_mut().bump().rc(), &mut temp_stack, type_map, partial_functions);
            let instructs = other_count.instruct_count;
            stack_count.add(&other_count);
            stack_count = stack_count.macro_instruct(&mut other_stack, Instruct::Comparator_If.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            (stack_count, other_stack, temp_stack)
        }
        Conditional::ElseIf { condition, statement } => {
            let mut other_stack = vec![];
            let mut stack_count = instruct_stack_expr(condition, scope_value.clone(), &mut other_stack, type_map, partial_functions);
            let mut temp_stack = vec![];
            let other_count = instruct_stack_statement(&statement.data, scope_value.borrow_mut().bump().rc(), &mut temp_stack, type_map, partial_functions);
            let instructs = other_count.instruct_count;
            stack_count.add(&other_count);
            stack_count = stack_count.macro_instruct(&mut other_stack, Instruct::Comparator_ElseIf.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            (stack_count, other_stack, temp_stack)

        }
        Conditional::Else { statement } => {
            let mut other_stack = vec![];
            let mut stack_count = StackCount::default();
            let mut temp_stack = vec![];
            let other_count = instruct_stack_statement(&statement.data, scope_value.borrow_mut().bump().rc(), &mut temp_stack, type_map, partial_functions);
            let instructs = other_count.instruct_count;
            stack_count.add(&other_count);
            stack_count = stack_count.macro_instruct(&mut other_stack, Instruct::Comparator_Else.instruct(instructs as u16, statement.location.line_number as u32), -1, vec![]);
            (stack_count, other_stack, temp_stack)

        }
    }
}

fn instruct_stack_expr_array(expr: &[&Expression], scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> StackCount {
    let mut stack_count = StackCount::default();
    expr.iter().for_each(|e| stack_count.add(&instruct_stack_expr(*e, scope_value.clone(), stack, type_map, partial_functions)));
    stack_count
}

fn instruct_stack_expr_vec(expr: &Vec<Expression>, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> StackCount {
    let mut stack_count = StackCount::default();
    expr.iter().for_each(|e| stack_count.add(&instruct_stack_expr(e, scope_value.clone(), stack, type_map, partial_functions)));
    stack_count
}


fn instruct_stack_expr(expr: &Expression, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>) -> StackCount {
    let line_number = expr.location.line_number as u32;
    match &expr.data {
        Expr::Break => { StackCount::default() }
        Expr::RangeExpression { start, bound, step } => {
            let stack_count = instruct_stack_expr_array(&[&start, &bound, &step], scope_value, stack, type_map, partial_functions);
            stack_count.macro_instruct(stack, Instruct::IteratorSetup(0).instruct(0, line_number), -2, vec![LeBlancType::Derived(DerivedType::Iterator)])
        }
        Expr::StaticMethodCall { method_name: method, args } => {
            let mut stack_count = StackCount::default();
            args.iter().for_each(|arg| stack_count.add(&instruct_stack_expr(arg, scope_value.clone(), stack, type_map, partial_functions)));
            println!("Stack count: {:#?}", stack_count);
            let arg_count = stack_count.clone();
            let mut args = arg_count.provided_types.len();
            let mut index_partial: Option<(usize, PartialFunction)> = None;
            println!("Searching: {:#?}", method);
            if let Expr::Ident { ident } = &method.data {
                let unpacked = unpack_ident(ident, type_map);
                let mut module = unsafe { CURRENT_MODULE.clone() };
                let mut name = String::new();
                let skip_latter = match unpacked.first() {
                    Some(unpack) => {
                        match unpack.ty {
                            UIdentType::Module => {
                                module = unpack.string.clone();
                                name = unpacked.last().unwrap().string.clone();
                                false
                            },
                            UIdentType::FuncOrVar => {
                                name = unpack.string.clone();
                                false
                            },
                            UIdentType::Object => {
                                let identity = curmod_ident_get(type_map, &IdentStore::Variable(unpack.string.clone())).unwrap().iter().find(|a| a.scope == scope_value.borrow().get_scope_type()).unwrap();
                                stack_count = stack_count.macro_instruct(stack, Instruct::LoadLocal.instruct(identity.id as u16, line_number), 1, identity.types.clone());
                                for packed in unpacked[1..].iter() {
                                    let arg = add_constant(Const::String(packed.string.clone()));
                                    stack_count = stack_count.macro_instruct(stack, Instruct::LoadAttr.instruct(arg as u16, line_number), 1, vec![LeBlancType::Flex]);
                                }
                                true
                            }
                            _ => {true} // This is an error
                        }
                    }
                    None => { true } // Another error
                };

                if !skip_latter {
                    println!("{} Provided types: {:#?}", name, stack_count.provided_types);
                    let partial = PartialFunction {
                        name: name,
                        args: LeBlancArgument::from_positional(&stack_count.provided_types),
                        returns: vec![]
                    };
                    args = partial.args.len();

                    index_partial = partial_functions.iter().cloned().enumerate().find(|(_index, p)| *p == partial);
                    let unwrapped = index_partial.as_ref().unwrap();
                    stack_count.expects(unwrapped.1.args.iter().filter_map(|a| {
                        if a.required {
                            Some(a.typing)
                        } else {
                            None
                        }
                    }).collect());

                    stack_count = stack_count.macro_instruct(stack,
                    LoadFunction.instruct(unwrapped.0 as u16, ident.location.line_number as u32), 1 - unwrapped.1.args.len() as isize, vec![LeBlancType::Function]);
                }//stack_count.add(&instruct_stack_ident(ident, scope_value, stack, type_map, partial_functions, InstructionBase::LoadFunction));
            } else {
                stack_count.add(&instruct_stack_expr(method, scope_value, stack, type_map, partial_functions));
            }

            let unwrapped = index_partial.unwrap();
            //stack.push(Instruction::new(Instruct::CallFunction, arg_count.stack_count as u16, expr.location.line_number as u32));
            stack_count.stack_count -= arg_count.stack_count;
            stack_count.macro_instruct(stack,Instruction::new(Instruct::CallFunction, args as u16, expr.location.line_number as u32),
            arg_count.stack_count - 1 + arg_count.special_count, unwrapped.1.returns)
        }
        Expr::ListIndex { list, slice } => {
            let mut stack_count = instruct_stack_expr(&Expression::new(expr.location,*slice.clone()), scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(list, scope_value, stack, type_map, partial_functions));
            stack_count.macro_instruct(stack, Instruct::ElementAccess.instruct(0, line_number), -1, vec![LeBlancType::Flex])
        }
        Expr::Slice { left, right } => {  // TODO: Implement Slice
            StackCount::default()
        }
        Expr::SeriesIndex { indices } => { // TODO: Implement
            if indices.len() == 1 {
                instruct_stack_expr_vec(indices, scope_value, stack, type_map, partial_functions)
            } else {
                let stack_count = instruct_stack_expr_vec(indices, scope_value, stack, type_map, partial_functions);
                let arg_count = (&stack_count).stack_count;
                stack_count.macro_instruct(stack, Instruct::ListSetup.instruct(arg_count as u16, line_number), 1-arg_count, vec![LeBlancType::Derived(DerivedType::List)])
            }
        }
        Expr::Equality { left, comparator, right } => {
            let mut stack_count = instruct_stack_expr(left, scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(right, scope_value, stack, type_map, partial_functions));
            stack_count.macro_instruct(stack, Instruct::Equality(0).instruct(comparator_instruct(*comparator), line_number), -1, vec![LeBlancType::Boolean])
        }
        Expr::List { items } => {
            let stack_count = instruct_stack_expr_vec(items, scope_value, stack, type_map, partial_functions);
            let arg_count = stack_count.stack_count as isize;
            let provided = stack_count.provided_types.len();
            stack_count.macro_instruct(stack, Instruct::ListSetup.instruct(provided as u16, line_number), 1-arg_count, vec![LeBlancType::Derived(DerivedType::List)])
        }
        Expr::ArithPlusMinusOperation { left, op, right } => {
            let mut stack_count = instruct_stack_expr(left, scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(right, scope_value, stack, type_map, partial_functions));
            let typing = *stack_count.provided_types.last().unwrap();
            stack_count.macro_instruct(stack, binary_instruct(*op).instruct(0, line_number), -1, vec![])
        }
        Expr::ArithMulDivModOperation { left, op, right } => {
            let mut stack_count = instruct_stack_expr(left,  scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(right, scope_value, stack, type_map, partial_functions));
            let typing = *stack_count.provided_types.last().unwrap();
            stack_count.macro_instruct(stack, binary_instruct(*op).instruct(0, line_number), -1, vec![])
        }
        Expr::ExponentialOperation { left, op, right } => {
            let mut stack_count = instruct_stack_expr(left, scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(right, scope_value, stack, type_map, partial_functions));
            let typing = *stack_count.provided_types.last().unwrap();
            stack_count.macro_instruct(stack, binary_instruct(*op).instruct(0, line_number), -1, vec![])
        }
        Expr::UnaryOperation { term, op } => {
            let stack_count = instruct_stack_expr(term, scope_value, stack, type_map, partial_functions);
            let typing = *stack_count.provided_types.last().unwrap();
            stack_count.macro_instruct(stack, unary_instruct(*op).instruct(0, line_number), 0, vec![typing])
        }
        Expr::IncrementDecrementOperation { term, op, postfix } => {
            let stack_count = instruct_stack_expr(term, scope_value, stack, type_map, partial_functions);
            let arg = if *postfix { 1 } else { 0 };
            let typing = *stack_count.provided_types.last().unwrap();
            stack_count.macro_instruct(stack, unary_instruct(*op).instruct(arg, line_number), 0, vec![typing])
        }
        Expr::ListAssignment { list, expr } => {
            let mut stack_count = instruct_stack_expr(list, scope_value.clone(), stack, type_map, partial_functions);
            stack_count.add(&instruct_stack_expr(list, scope_value, stack, type_map, partial_functions));
            stack.pop(); //Important
            stack_count.macro_instruct(stack, Instruct::ElementStore.instruct(0, line_number), -1, vec![])
        }
        Expr::TypedAssignment { idents, expr } => {
            let mut stack_count = StackCount::default();
            for ident in idents {
                if let Expr::TypedVariable { typing, variable } = ident.data.clone() {
                    match expr {
                        Some(expr) => {
                            stack_count.add(&instruct_stack_expr(expr, scope_value.clone(), stack, type_map, partial_functions));
                            let identity = curmod_ident_get(type_map, &IdentStore::Variable(variable)).unwrap().iter().find(|a| a.scope == scope_value.borrow().get_scope_type()).unwrap();
                            stack_count.expects(identity.types.clone());
                            stack_count = stack_count.macro_instruct(stack,
                           Instruct::StoreLocal.instruct(identity.id as u16, ident.location.line_number as u32), -1, vec![]);
                        }
                        None => {}
                    }
                }
            }
            stack_count
        }
        Expr::NormalAssignment { idents, expr } => {
            let mut stack_count = StackCount::default();
            for ident in idents {
                stack_count.add(&instruct_stack_expr(&expr.clone(), scope_value.clone(), stack, type_map, partial_functions));
                stack_count.add(&instruct_stack_ident(ident, scope_value.clone(), stack, type_map, partial_functions, Instruct::StoreLocal));
            }
            stack_count
        }
        Expr::GroupAssignment { assignee, group } => {
            let mut stack_count = StackCount::default();
            stack_count.add(&instruct_stack_expr(assignee, scope_value.clone(), stack, type_map, partial_functions));
            stack_count.add(&instruct_stack_expr(group, scope_value, stack, type_map, partial_functions));
            stack_count.expect(LeBlancType::Group);
            stack_count.macro_instruct(stack, Instruct::Group.instruct(0, line_number), -1, vec![LeBlancType::Promise])
        }
        Expr::BlockLambda { variables, block } => {
            // TODO
            StackCount::default()
        }
        Expr::ExprLambda { variables, expr } => {
            // TODO
            StackCount::default()
        }
        Expr::ExceptCatch { .. } => {
            // TODO
            StackCount::default()
        }
        Expr::TypedVariable { typing, variable } => {
            let stack_count = StackCount::default();
            let identity = curmod_ident_get(type_map, &IdentStore::Variable(variable.clone())).unwrap().iter().find(|a| a.scope == scope_value.borrow().get_scope_type()).unwrap();
            stack_count.macro_instruct(stack,
           LoadLocal.instruct(identity.id as u16, line_number), -1, identity.arg_types.clone())

        }
        Expr::Ident { ident } => {
            instruct_stack_ident(ident, scope_value, stack, type_map, partial_functions, Instruct::LoadLocal)
        }
        Expr::Constant { constant } => {
            let arg = add_constant(constant.clone());
            StackCount::default().macro_instruct(stack, Instruct::LoadConstant.instruct(arg as u16, line_number), 1, vec![constant.to_lb_type(arg as u32)])
        }
        _ => { StackCount::default() }
    }
}
fn instruct_stack_ident(ident: &Ident, scope_value: Rc<RefCell<ScopeTrack>>, stack: &mut Vec<Instruction>, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>, partial_functions: &mut Vec<PartialFunction>, instruct: InstructionBase) -> StackCount {
    match &ident.data {
        Id::Ident { ident: id } => {
            let stack_count = StackCount::default();
            let scopes = curmod_ident_get(type_map, &IdentStore::Variable(id.clone()));
            if scopes.is_none() {
                let (store, set) = type_map.get_mut("_MAIN_").unwrap().iter().find(|(s, v)| s.get_ident() == id).unwrap();
                let identity = set.get_first_id().unwrap();
                stack_count.macro_instruct(stack,
                Instruct::LoadFunction.instruct(identity as u16, ident.location.line_number as u32), -1, vec![LeBlancType::Function])
            } else {
                let identity = curmod_ident_get(type_map,&IdentStore::Variable(id.clone())).unwrap().get_any_scope_value(&scope_value.borrow().get_scope_type()).unwrap();
                stack_count.macro_instruct(stack,
                instruct.instruct(identity.id as u16, ident.location.line_number as u32), -1, identity.types.clone())
            }
        }
        Id::ObjIdent { ident: id, attr } => {
            // TODO
            StackCount::default()
        }
        Id::EnumIdent { .. } => {
            StackCount::default()
        }
        Id::TypedListIdent { .. } => {
            StackCount::default()
        }
    }
}

fn unpack_ident(ident: &Ident, type_map: &mut HashMap<String, HashMap<IdentStore, ScopeSet>>) -> Vec<UnpackedIdent> {
    let mut parts: Vec<UnpackedIdent> = vec![];
    match &ident.data {
        Id::Ident { ident } => {
            let ty = match parts.first() {
                Some(val) => {
                    match val.ty {
                        UIdentType::Object => UIdentType::Attr,
                        UIdentType::Module => UIdentType::FuncOrVar,
                        _ => UIdentType::Attr
                    }
                },
                None => {
                    let scope_map = unsafe {type_map.get_mut(&CURRENT_MODULE)}.unwrap();
                    match scope_map.iter().find(|(store, set)| store.get_ident() == ident) {
                        Some(_) => UIdentType::Object, // We tune this afterwards
                        None => {
                            if type_map.contains_key(ident) { UIdentType::Module }
                            else { UIdentType::FuncOrVar }
                        }
                    }
                }
            };
            parts.push( UnpackedIdent { string: ident.to_owned(), ty })
        }
        Id::ObjIdent { ident, attr } => {
            parts.append(&mut unpack_ident(ident, type_map));
            parts.append(&mut unpack_ident(attr, type_map));
        }
        Id::EnumIdent { .. } => {}
        Id::TypedListIdent { .. } => {}
    }
    if parts.len() == 1 {
        let part = parts.get_mut(0).unwrap();
        if part.ty == UIdentType::Object {
            part.ty = UIdentType::FuncOrVar;
        }
    }
    parts
}

#[derive(PartialEq, Debug)]
pub struct UnpackedIdent {
    pub string: String,
    pub ty: UIdentType
}

#[derive(PartialEq, Debug)]
pub enum UIdentType {
    Object,
    Module,
    Attr,
    FuncOrVar
}


#[derive(Default, Clone, Debug)]
pub struct StackCount {
    pub instruct_count: usize,
    pub stack_count: isize,
    pub special_count: isize,
    pub constants: HashMap<Const, usize>,
    pub needed_types: Vec<LeBlancType>,
    pub provided_types: Vec<LeBlancType>,
    pub errors: Vec<SyntaxError>
}

impl StackCount {
    pub fn add(&mut self, other: &StackCount) {
        self.instruct_count += other.instruct_count;
        self.stack_count += other.stack_count;
        self.special_count += other.special_count;
        self.constants.extend(other.constants.clone().into_iter());
        self.needed_types.append(&mut other.needed_types.clone());
        self.provided_types.append(&mut other.provided_types.clone());
        self.errors.append(&mut other.errors.clone());
    }

    pub fn expect(&mut self, lbt: LeBlancType) {
        self.needed_types.push(lbt);
    }

    pub fn expects(&mut self, mut lbt: Vec<LeBlancType>) {
        self.needed_types.append(&mut lbt);
    }

    pub fn macro_instruct(mut self, instructions: &mut Vec<Instruction>, instruct: Instruction, mut stack_count: isize, mut pushed_types: Vec<LeBlancType>) -> Self {
        self.instruct_count += 1;

        match stack_count.partial_cmp(&0).unwrap() {
            Ordering::Less => stack_count = stack_count.abs(),
            Ordering::Equal => stack_count = 1,
            Ordering::Greater => stack_count = 0
        }

        for _ in 0..stack_count {
            let t =self.provided_types.pop();



            if !self.needed_types.is_empty() {
                let needed_type = self.needed_types.pop().unwrap();
                if t.is_some() {
                    if let LeBlancType::ConstantFlex(arg) = t.as_ref().unwrap() {
                        let const_type = get_constant(*arg as usize).unwrap().clone();
                        match const_type {
                            Const::Whole(v, t) => set_constant(*arg as usize, Const::Whole(v, Some(needed_type))),
                            Const::Float(v, t) => set_constant(*arg as usize, Const::Float(v, Some(needed_type))),
                            _ => {}
                        }
                    } else if t.unwrap() != needed_type {
                        self.errors.push(SyntaxError {} );
                        println!("ERROR!!!!");
                    }
                } else {
                    self.errors.push(SyntaxError {} );
                    println!("ERROR!!!!");
                }
            }
        }
        if !self.errors.is_empty() {
            return self
        }
        self.provided_types.append(&mut pushed_types);

        self.stack_count += stack_count;
        instructions.push(instruct);
        self
    }
}

pub struct ConstantTrack {
    pub constants: Vec<(Const, usize)>
}

impl ConstantTrack {
    pub fn constant(&mut self, constant: Const) -> usize {
        println!("const: {:?} Constants: {:?}", constant, self.constants);
        match self.constants.iter().find(|(con, v)| *con == constant) {
            Some((con, v)) => *v,
            None => {
                let ret = self.constants.len();
                self.constants.push((constant, ret));
                ret
            }
        }
    }
}

fn add_constant(constant: Const) -> usize {
    unsafe {
        if CONSTANT_TRACK.is_none() { CONSTANT_TRACK = Some(ConstantTrack { constants: Vec::default() }) }
        CONSTANT_TRACK.as_mut().unwrap().constant(constant)
    }
}

fn get_constant(index: usize) -> Option<&'static Const> {
    unsafe {
        if CONSTANT_TRACK.is_none() { CONSTANT_TRACK = Some(ConstantTrack { constants: Vec::default() }) }
        CONSTANT_TRACK.as_ref().unwrap().constants.iter().find_map(|(con, val)| {
            if *val == index { Some(con) } else { None }
        })
    }
}

fn set_constant(index: usize, value: Const) {
    unsafe {
        if CONSTANT_TRACK.is_none() { CONSTANT_TRACK = Some(ConstantTrack { constants: Vec::default() }) }
        let val = CONSTANT_TRACK.as_ref().unwrap().constants.iter().position(|(con, val)| { *val == index }).unwrap();
        let mut_ref = CONSTANT_TRACK.as_mut().unwrap();
        mut_ref.constants.remove(val);
        mut_ref.constants.push((value, index));
    }
}

fn sorted_vec() -> Vec<Const> {
    unsafe {
        if CONSTANT_TRACK.is_none() { return vec![] }
        let mut val = &mut CONSTANT_TRACK.as_mut().unwrap().constants.iter().collect::<Vec<&(Const, usize)>>();
        val.sort_by(|(_c1, i1), (_c2, i2)| i1.cmp(i2));
        let val = val.into_iter().map(|(c, _i)| (*c).clone()).collect::<Vec<Const>>();
        val
    }
}

fn curmod_ident_get<'a>(map: &'a mut HashMap<String, HashMap<IdentStore, ScopeSet>>, store: &IdentStore) -> Option<&'a mut ScopeSet> {
    map.get_mut(unsafe { &CURRENT_MODULE }).unwrap().get_mut(store)
}