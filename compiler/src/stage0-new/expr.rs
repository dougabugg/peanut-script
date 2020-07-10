pub type Var = usize;

#[derive(Clone, Copy)]
pub struct Span<T> {
    // TODO info for source map
    pub span: (),
    pub inner: T,
}

pub enum Expr {
    LiteralValue(Span<LiteralValue>),
    LocalScope(Span<Var>),
    ModuleScope(Span<usize>),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    SeqIndex {
        seq: Box<Expr>,
        index: Box<Expr>,
    },
    SeqLen {
        seq: Box<Expr>,
    },
    SeqToList {
        seq: Box<Expr>,
    },
    TupleCreate(Vec<Expr>),
    TupleFromList(Box<Expr>),
    TableCreate(Box<Expr>),
    ListCreate(Vec<Expr>),
    ListGetSlice {
        list: Box<Expr>,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    ListPop(Box<Expr>),
    BufferCreate(Box<Expr>),
    BufferGetSlice {
        buffer: Box<Expr>,
        a: Box<Expr>,
        b: Box<Expr>,
    },
}

impl Expr {
    pub fn find_vars(&self) -> Vec<Span<Var>> {
        let mut vars = Vec::new();
        self.acc_vars(&mut vars);
        vars
    }

    fn acc_vars(&self, vars: &mut Vec<Span<Var>>) {
        match self {
            Expr::LocalScope(var) => vars.push(var),
            Expr::BinaryOp(b) => {
                b.lhs.acc_vars(vars);
                b.rhs.acc_vars(vars);
            },
            Expr::UnaryOp(u) => u.expr.acc_vars(vars),
            Expr::Call { func, args } => {
                func.acc_vars(vars);
                for arg in args {
                    arg.acc_vars(vars);
                }
            },
            Expr::SeqIndex { seq, index } => {
                seq.acc_vars(vars);
                index.acc_vars(vars);
            },
            Expr::SeqLen { seq } => seq.acc_vars(vars),
            Expr::SeqToList { seq } => seq.acc_vars(vars),
            Expr::TupleCreate(exprs) => {
                for e in exprs {
                    e.acc_vars(vars);
                }
            },
            Expr::TupleFromList(e) => e.acc_vars(vars),
            Expr::TableCreate(e) => e.acc_vars(vars),
            Expr::ListCreate(exprs) => {
                for e in exprs {
                    e.acc_vars(vars);
                }
            },
            Expr::ListGetSlice { list, a, b } => {
                list.acc_vars(vars);
                a.acc_vars(vars);
                b.acc_vars(vars);
            },
            Expr::ListPop(e) => e.acc_vars(vars),
            Expr::BufferCreate(e) => e.acc_vars(vars),
            Expr::BufferGetSlice { buffer, a, b } => {
                buffer.acc_vars(vars);
                a.acc_vars(vars);
                b.acc_vars(vars);
            },
        }
    }
}

pub struct BinaryOp {
    pub op_type: BinaryOpType,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[rustfmt::skip]
pub enum BinaryOpType {
    Add, Sub, Mul, Div, Rem, Shl, Shr, And, Or, Xor,
    Equal, NotEqual, Greater, GreaterOrEqual, Less, LessOrEqual,
    Identity, LogicAnd, LogicOr
}

pub struct UnaryOp {
    pub op_type: UnaryOpType,
    pub expr: Box<Expr>,
}

#[rustfmt::skip]
pub enum UnaryOpType {
    Neg, Not, LogicNot, IntToReal, Floor, Ceil, Trunc, Round
}
