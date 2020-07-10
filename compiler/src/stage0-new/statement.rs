pub enum Statement {
    BindLocal(usize),
    DropLocal(usize),
    InitLocal(usize),
    Loop(Loop),
    Break { label: Option<usize> },
    Continue { label: Option<usize> },
    Expr(Expr),
    Return(Expr),
    IfElse(IfElse),
    Assign { place: Box<Expr>, value: Box<Expr> },
    SeqAppend { seq: Box<Expr>, src: Box<Expr> },
    SeqResize { seq: Box<Expr>, len: Box<Expr> },
    ListPush { list: Box<Expr>, value: Box<Expr> },
    BufferSetSlice(Box<BufferSetSlice>),
}

pub struct Loop {
    condition: Option<Expr>,
    label: Option<usize>,
    body: Vec<Statement>,
}

pub struct IfElse {
    pub if_: If,
    pub else_if: Vec<If>,
    pub else_: Vec<Statement>,
}

pub struct If {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

pub struct BufferSetSlice {
    pub buffer: Expr,
    pub src: Expr,
    pub src_offset: Expr,
    pub offset: Expr,
    pub len: Expr,
}
