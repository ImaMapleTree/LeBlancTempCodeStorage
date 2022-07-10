
static mut BYTE_LOCATION: Vec<(usize, usize)> = vec![];

#[macro_use]
macro_rules! make_ast {
    ($l:expr, $r:expr, $name:ident, $data:expr) => {
        $name::new(Location::new(($l, $r)), $data)
    }

}

pub(crate) use make_ast;

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

#[derive(Clone, Debug)]
pub enum Cmpnt {
    Function { header: Box<Component>, body: Statement, tags: Vec<String> },
    FunctionHeader { name: String, args: Vec<Expression>, returns: Vec<ParseType> },
    Class { name: String, super_traits: Vec<String>, items: Vec<Component> },
    Trait { name: String, super_traits: Vec<String>, items: Vec<Component> },
    Extension { name: String, targets: Vec<ParseType>, items: Vec<Component> },
    Property { typing: ParseType, ident: String, value: Option<Expression> },
    Import { module: String, import: Option<Vec<String>>},
    ExtImport { module: String, extension: String },
    Enum { name: String, type_params: Option<Vec<String>>, items: Vec<Component> },
    EnumItem { name: String, nested: Vec<ParseType> }
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

    DeclarationAssignment {
        prefix: Box<Expression>,
        expr: Option<Box<Expression>>,
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
        errors: Vec<ParseType>,
        variable: String,
    },

    TypedVariable { typing: ParseType, variable: String },

    Ident { ident: Ident },

    Constant {constant: Const }
}

#[derive(Clone, Debug)]
pub enum Id {
    Ident { ident: String },
    ObjIdent { ident: Box<Ident>, attr: Box<Ident>},
    EnumIdent {ident: Box<Ident>, kind: Box<Ident>}
}


#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    UPositive,
    UNegative,
    UNot,
    UInverse,
    UIncrement,
    UDecrement
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Const {
    String(String),
    Whole(i128),
    Float(f64),
    Boolean(bool)
}

#[derive(Clone, Debug)]
pub enum ParseType {
    Flex,
    String,
    Int,
    Float,
    Double,
    Function,
    Group,
    Promise,
    SelfRef,
    SuperLambda,
    Class(String),
    Trait(String, bool)
}