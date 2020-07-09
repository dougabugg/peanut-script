pub enum SharedStatement<S, E> {
    Expr(E),
    Return(E),
    IfElse(IfElse<S, E>),
    SeqAppend { seq: E, src: E },
    SeqResize { seq: E, len: E },
    ListPush { list: E, value: E },
    BufferSetSlice(Box<BufferSetSlice<E>>),
}

pub struct IfElse<S, E> {
    pub if_: If<S, E>,
    pub else_if: Vec<If<S, E>>,
    pub else_: Vec<S>,
}

pub struct If<S, E> {
    pub condition: E,
    pub body: Vec<S>,
}

pub struct BufferSetSlice<E> {
    pub buffer: E,
    pub src: E,
    pub src_offset: E,
    pub offset: E,
    pub len: E,
}
