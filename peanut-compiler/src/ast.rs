pub enum Literal {
    None,
    Bool(bool),
    Integer(i64),
    Real(f64),
    String(usize),
}

pub enum Expr {
    Literal(Literal),
    LocalScope(usize),
    ModuleScope(usize),
    Assign {
        place: Box<Expr>,
        value: Box<Expr>,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    SeqIndex {
        seq: Box<Expr>,
        index: Box<Expr>,
    },
    SeqSlice {
        seq: Box<Expr>,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    SeqResize {
        seq: Box<Expr>,
        size: Box<Expr>,
    },
    SeqAppend {
        seq: Box<Expr>,
        src: Box<Expr>,
    },
    SeqToList(Box<Expr>),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    TupleCreate(Vec<Expr>),
    TupleFromList(Box<Expr>),
    RecordCreate(Vec<Expr>),
    RecordFromList(Box<Expr>),
    TableCreate(Vec<(Expr, Expr)>),
    TableFromList(Box<Expr>),
    ListCreate(Vec<Expr>),
    ListPush {
        list: Box<Expr>,
        expr: Box<Expr>,
    },
    ListPop(Box<Expr>),
    BufferCreate(Vec<Expr>),
}

pub enum BinaryOp {
    Add, Sub, Mul, Div, Rem, Shl, Shr, And, Or, Xor,
    Equal, NotEqual, Greater, GreaterOrEqual, Less, LessOrEqual,
    Identity
}

pub enum UnaryOp {
    Neg, Not, Floor, Ceil, Trunc, Round
}

pub enum Statement {
    Expr(Expr),
    DeclareLocal(usize),
    Return(Expr),
    IfElse(IfElse),
    Loop {
        label: Option<usize>,
        body: Vec<Statement>,
    },
    Break {
        label: Option<usize>,
    },
    Continue {
        label: Option<usize>,
    },
}

pub struct IfElse {
    if_: If,
    else_if: Vec<If>,
    else_: Vec<Statement>,
}

pub struct If {
    condition: Expr,
    body: Vec<Statement>,
}

pub struct Function {
    args: Vec<usize>,
    body: Vec<Statement>,
}

pub struct Module {
    items: Vec<ModuleItem>,
}

pub enum ModuleItem {
    ModuleRef(usize),
    Function(Function),
    Literal(Literal),
}

pub struct Program {
    modules: Vec<Module>,
}
