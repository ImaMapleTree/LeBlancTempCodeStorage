use core::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use enum_as_inner::EnumAsInner;
use crate::leblanc::core::native_types::LeBlancType;

use serde::{Serialize};

use crate::leblanc::rustblanc::hex::Hexadecimal;
use crate::leblanc::rustblanc::Hexable;
use crate::leblanc::rustblanc::lazy_store::{Lazy, Strategy};
use crate::leblanc::rustblanc::path::ZCPath;


static mut BYTE_LOCATION: Vec<(usize, usize)> = vec![];
static mut FILE: ZCPath = ZCPath::constant();





pub unsafe fn push_byte_location(value: (usize, usize)) {
    BYTE_LOCATION.push(value);
}

pub unsafe fn clear_byte_location() {
    BYTE_LOCATION.clear();
}

pub unsafe fn set_file<T: Display>(file: T) {
    FILE = ZCPath::new(file);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Serialize, Default)]
pub struct Location {
    pub file: ZCPath,
    pub byte_pos: (usize, usize),
    pub line: usize,
    pub symbol: usize,
}

impl Location {
    pub fn new(byte_pos: (usize, usize)) -> Location {
        let loc = unsafe { BYTE_LOCATION[byte_pos.0] };
        let file = unsafe { FILE };
        Location {
            file,
            byte_pos,
            line: loc.0,
            symbol: loc.1
        }
    }

    pub fn builtin() -> Location {
        Location {
            file: ZCPath::new("__BUILTIN__"),
            byte_pos: Default::default(),
            line: Default::default(),
            symbol: Default::default()
        }
    }
}

impl<T: Serialize> Display for Located<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
pub struct Located<T> {
    #[serde(skip_serializing)]
    pub location: Location,
    #[serde(flatten)]
    pub data: T,
}

impl<T: Debug> Located<T> {
    pub fn new(location: Location, data: T) -> Self {
        Self {
            location,
            data,
        }
    }
}

impl Default for Component {
    fn default() -> Self {
        Located {
            location: Default::default(),
            data: Default::default()
        }
    }
}

pub type Component = Located<Cmpnt>;
pub type Expression = Located<Expr>;
pub type Statement = Located<Stmnt>;
pub type Ident = Located<Id>;
pub type Constant = Located<Const>;
pub type Required = Located<Reqs>;
pub type LConditional = Located<Conditional>;

impl Default for Ident {
    fn default() -> Self {
        Ident {
            location: Location {
                file: Default::default(),
                byte_pos: (0, 0),
                line: 0,
                symbol: 0
            },
            data: Id::Ident {
                ident: String::default()
            }
        }
    }
}

impl Display for Cmpnt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


///`Function`
/// ## header: `Box<Component>`, body: Statement, tags: `Vec<String>`
///`FunctionHeader`
/// ## name: String, args: `Vec<Expression>`, returns: `Vec<LeBlancType>`
///`Class`
/// ## name: String, super_traits: `Vec<String>`, items: `Vec<Component>`
///`Trait`
/// ## name: String, super_traits: `Vec<String>`, items: `Vec<Component>`
///`Extension`
/// ## name: String, targets: `Vec<LeBlancType>`, items: `Vec<Component>`
///`Property`
/// ## typing: LeBlancType, ident: String, value: `Option<Expression>`
///`Import`
/// ## module: String, import: `Option<Vec<String>>`
///`ExtImport`
/// ## module: String, extension: String
///`Enum`
/// ## name: String, type_params: `Option<Vec<String>>`, items: `Vec<Component>`
///`EnumItem`
/// ## name: String, nested: `Vec<LeBlancType>`
#[derive(Clone, Serialize, Debug, Default, EnumAsInner)]
#[serde(tag = "type")]
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
    EnumItem { name: String, nested: Vec<LeBlancType> },
    Requirements { required: Vec<Required> },
    #[default]
    None
}



impl Display for Stmnt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


#[derive(Clone, PartialEq, Eq, Serialize, Debug, Default)]
#[serde(untagged)]
pub enum Stmnt {
    Global {
        statement: Box<Statement>
    },

    Block { statements: Vec<Statement> },
    Line {
        #[serde(flatten)]
        expr: Expression
    },

    Conditional { conditional: Conditional },
    MultiConditional { conditionals: Vec<Statement> },
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

    Return { statement: Box<Statement> },
    #[default]
    None
}

impl Display for Conditional {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Debug)]
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

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Debug)]
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

    /*ClassMethodCall {
        class: Box<Expression>,
        method_name: Ident,
        args: Vec<Expression>
    },*/

    ListIndex {
        list: Box<Expression>,
        slice: Box<Expression>
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
        #[serde(flatten)]
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

    TypedVariable {
        typing: LeBlancType,
        #[serde(flatten)]
        variable: Ident
    },

    Ident {
        #[serde(flatten)]
        ident: Ident
    },

    Constant {
        constant: Const
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Id::Ident { ident } => ident.to_string(),
            Id::ObjIdent { ident, attr } => ident.resolve() + "." + &attr.resolve(),
            Id::EnumIdent { ident, kind } => ident.resolve() + "." + &kind.resolve(),
            Id::TypedListIdent { typing } => typing.to_string()
        };
        write!(f, "{}", s)
    }
}

impl Ident {
    pub fn resolve(&self) -> String {
        match &self.data {
            Id::Ident { ident } => ident.to_string(),
            Id::ObjIdent { ident, attr} => ident.resolve() + "." + &attr.resolve(),
            Id::EnumIdent { ident, kind } => ident.resolve() + "." + &kind.resolve(),
            Id::TypedListIdent { typing } => typing.to_string()
        }
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
#[serde(untagged)]
pub enum Id {
    Ident { ident: String },
    ObjIdent { ident: Box<Ident>, attr: Box<Ident>},
    EnumIdent {ident: Box<Ident>, kind: Box<Ident>},
    TypedListIdent { typing: LeBlancType },
}

impl Default for Id {
    fn default() -> Self {
        Id::Ident { ident: Default::default() }
    }
}


#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
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

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
pub enum UnaryOperator {
    UPositive,
    UNegative,
    UNot,
    UInverse,
    UIncrement,
    UDecrement
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
pub enum Comparator {
    Equal,
    NotEqual,
    GreaterEqual,
    LesserEqual,
    Greater,
    Lesser,
    In
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum CondType {
    If,
    ElseIf,
    Else
}

impl Debug for Const {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub enum Reqs {
    Requires { dependees: Vec<Ident>, required: Vec<Ident> },
    Export { exports: Vec<Ident> }
}


#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Const {
    String(String, Location),
    Whole(i64, Option<LeBlancType>, Location),
    Float(f64, Option<LeBlancType>, Location),
    Boolean(bool, Location),
}

impl Default for Const {
    fn default() -> Self {
        Self::Boolean(false, Default::default())
    }
}

//noinspection RsExternalLinter
impl Hash for Const {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Const::String(str, ..) => str.hash(state),
            Const::Whole(num, ..) => num.hash(state),
            Const::Float(num, ..) => num.to_string().hash(state),
            Const::Boolean(bool, ..) => bool.hash(state),
        }
    }
}

impl Eq for Const {}

impl Const {
    pub fn location(&self) -> Location {
        match self {
            Const::String(_, location) => *location,
            Const::Whole(_, _, location) => *location,
            Const::Float(_, _, location) => *location,
            Const::Boolean(_, location) => *location
        }
    }

    pub fn to_lb_type(&self) -> LeBlancType {
        match self {
            Const::String(_, _location) => LeBlancType::String,
            Const::Whole(_, opt, _location) => {
                match opt {
                    None => LeBlancType::Int,
                    Some(lbt) => *lbt
                }
            }
            Const::Float(_, opt, _location) => {
                match opt {
                    None => LeBlancType::Double,
                    Some(lbt) => *lbt,
                }
            }
            Const::Boolean(_, _location) => LeBlancType::Boolean,
        }
    }

    pub fn to_hex(&self) -> Hexadecimal {
        match self {
            Const::String(str, _) => str.to_hex(0),
            Const::Boolean(truth, _) => truth.to_hex(128),
            Const::Whole(val, t, _) => {
                match t {
                    None => (*val as i32).to_hex(128),
                    Some(ty) => ty.transform(val.to_string()),
                }
            }
            Const::Float(val, t, _) => {
                match t {
                    None => (*val as f64).to_hex(128),
                    Some(ty) => ty.transform(val.to_string()),
                }
            }
        }
    }
}

impl Lazy for Const {
    fn scan(&self, other: &Self, _strategy: Strategy) -> bool {
        self == other
    }
}