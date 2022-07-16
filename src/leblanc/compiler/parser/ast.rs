
static mut BYTE_LOCATION: Vec<(usize, usize)> = vec![];

#[macro_use]
macro_rules! make_ast {
    ($l:expr, $r:expr, $name:ident, $data:expr) => {
        $name::new(Location::new(($l, $r)), $data)
    }

}

use std::hash::{Hash, Hasher};
pub(crate) use make_ast;
use crate::leblanc::core::native_types::LeBlancType;
use crate::leblanc::core::native_types::LeBlancType::ConstantFlex;

pub unsafe fn push_byte_location(value: (usize, usize)) {
    BYTE_LOCATION.push(value);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub byte_pos: (usize, usize),
    pub line_number: usize,
    pub symbol_number: usize,
}

impl Location {
    pub fn new(byte_pos: (usize, usize)) -> Location {
        let loc = unsafe { BYTE_LOCATION[byte_pos.0] };
        Location {
            byte_pos,
            line_number: loc.0,
            symbol_number: loc.1
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub data: T,
}

impl<T> Located<T> {
    pub fn new(location: Location, data: T) -> Self {
        Self {
            location,
            data,
        }
    }
}


pub type Component = Located<Cmpnt>;
pub type Expression = Located<Expr>;
pub type Statement = Located<Stmnt>;
pub type Ident = Located<Id>;
pub type Constant = Located<Const>;

impl Default for Ident {
    fn default() -> Self {
        Ident {
            location: Location {
                byte_pos: (0, 0),
                line_number: 0,
                symbol_number: 0
            },
            data: Id::Ident {
                ident: String::default()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cmpnt {
    Function { header: Box<Component>, body: Statement, tags: Vec<String> },
    FunctionHeader { name: String, args: Vec<Expression>, returns: Vec<LeBlancType> },
    Class { name: String, super_traits: Vec<String>, items: Vec<Component> },
    Trait { name: String, super_traits: Vec<String>, items: Vec<Component> },
    Extension { name: String, targets: Vec<LeBlancType>, items: Vec<Component> },
    Property { typing: LeBlancType, ident: String, value: Option<Expression> },
    Import { module: String, import: Option<Vec<String>>},
    ExtImport { module: String, extension: String },
    Enum { name: String, type_params: Option<Vec<String>>, items: Vec<Component> },
    EnumItem { name: String, nested: Vec<LeBlancType> }
}



#[derive(Clone, Debug)]
pub enum Stmnt {
    Global {
        statement: Box<Statement>
    },

    Block { statements: Vec<Statement> },
    Line { expr: Expression },

    Conditional { conditional: Conditional },
    While {
        condition: Expression,
        statement: Box<Statement>
    },
    For {
        variable: Expression, // TypedVariable
        iterable: Expression,
        statement: Box<Statement>
    },
    InfLoop {
        statement: Box<Statement>
    },

    Try { statement: Box<Statement> },
    Except { catch: Option<Expression>, statement: Box<Statement> },

    Return { statement: Box<Statement> }
}

#[derive(Clone, Debug)]
pub enum Conditional {
    If {
        condition: Expression,
        statement: Box<Statement>
    },
    ElseIf {
        condition: Expression,
        statement: Box<Statement>
    },
    Else {
        statement: Box<Statement>
    }
}



#[derive(Clone, Debug)]
pub enum Expr {
    Break,

    RangeExpression {
        start: Box<Expression>,
        bound: Box<Expression>,
        step: Box<Expression>
    },

    StaticMethodCall {
        method_name: Box<Expression>,
        args: Vec<Expression>
    },

    ClassMethodCall {
        class: Box<Expression>,
        method_name: Ident,
        args: Vec<Expression>
    },

    ListIndex {
        list: Box<Expression>,
        slice: Box<Expr>
    },

    Slice {
        left: Box<Expression>,
        right: Box<Expression>
    },

    SeriesIndex {
        indices: Vec<Expression>
    },

    Equality {
        left: Box<Expression>,
        comparator: Comparator,
        right: Box<Expression>
    },

    List {
        items: Vec<Expression>,
    },

    ArithPlusMinusOperation {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>
    },

    ArithMulDivModOperation {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>
    },

    ExponentialOperation {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>
    },

    UnaryOperation {
        term: Box<Expression>,
        op: UnaryOperator,
    },

    IncrementDecrementOperation {
        term: Box<Expression>,
        op: UnaryOperator,
        postfix: bool
    },
    ListAssignment {
        list: Box<Expression>,
        expr: Box<Expression>
    },

    TypedAssignment {
        idents: Vec<Expression>,
        expr: Option<Box<Expression>>
    },

    NormalAssignment {
        idents: Vec<Ident>,
        expr: Box<Expression>,
    },

    GroupAssignment {
        assignee: Box<Expression>,
        group: Box<Expression>
    },

    BlockLambda {
        variables: Vec<Ident>,
        block: Box<Statement>,
    },

    ExprLambda {
        variables: Vec<Ident>,
        expr: Box<Expression>,
    },

    ExceptCatch {
        errors: Vec<LeBlancType>,
        variable: String,
    },

    TypedVariable { typing: LeBlancType, variable: Ident },

    Ident { ident: Ident },

    Constant {constant: Const }
}

#[derive(Clone, Debug)]
pub enum Id {
    Ident { ident: String },
    ObjIdent { ident: Box<Ident>, attr: Box<Ident>},
    EnumIdent {ident: Box<Ident>, kind: Box<Ident>},
    TypedListIdent { typing: LeBlancType },
}


#[derive(Clone, Debug, Copy)]
pub enum BinaryOperator {
    BinAdd,
    BinSub,
    BinMul,
    BinDiv,
    BinPow,
    BinMod,
    BinLShift,
    BinRShift
}

#[derive(Clone, Debug, Copy)]
pub enum UnaryOperator {
    UPositive,
    UNegative,
    UNot,
    UInverse,
    UIncrement,
    UDecrement
}

#[derive(Clone, Debug, Copy)]
pub enum Comparator {
    Equal,
    NotEqual,
    GreaterEqual,
    LesserEqual,
    Greater,
    Lesser,
    In
}

#[derive(Clone, Debug)]
pub enum CondType {
    If,
    ElseIf,
    Else
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    String(String),
    Whole(i128, Option<LeBlancType>),
    Float(f64, Option<LeBlancType>),
    Boolean(bool),
}

impl Hash for Const {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Const::String(str) => str.hash(state),
            Const::Whole(num, _) => num.hash(state),
            Const::Float(num, _) => num.to_string().hash(state),
            Const::Boolean(bool) => bool.hash(state),
        }
    }
}

impl Eq for Const {}

impl Const {
    pub fn to_lb_type(&self, arg: u32) -> LeBlancType {
        match self {
            Const::String(_) => LeBlancType::String,
            Const::Whole(_, opt) => {
                match opt {
                    None => ConstantFlex(arg),
                    Some(lbt) => *lbt
                }
            }
            Const::Float(_, opt) => {
                match opt {
                    None => ConstantFlex(arg),
                    Some(lbt) => *lbt,
                }
            }
            Const::Boolean(_) => LeBlancType::Boolean,
        }
    }
}