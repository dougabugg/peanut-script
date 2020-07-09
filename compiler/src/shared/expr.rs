use crate::vm::bytecode::ops::LiteralValue;

pub enum SharedExpr<E> {
    LiteralValue(LiteralValue),
    ModuleScope(usize),
    BinaryOp(BinaryOp<E>),
    UnaryOp(UnaryOp<E>),
    Call {
        function: Box<E>,
        args: Vec<E>,
    },
    Assign {
        place: Box<E>,
        value: Box<E>,
    },
    SeqIndex {
        seq: Box<E>,
        index: Box<E>,
    },
    SeqLen {
        seq: Box<E>,
    },
    SeqToList {
        seq: Box<E>,
    },
    TupleCreate(Vec<E>),
    TupleFromList(Box<E>),
    TableCreate(Box<E>),
    ListCreate(Vec<E>),
    ListGetSlice {
        list: Box<E>,
        a: Box<E>,
        b: Box<E>,
    },
    ListPop(Box<E>),
    BufferCreate(Box<E>),
    BufferGetSlice {
        buffer: Box<E>,
        a: Box<E>,
        b: Box<E>,
    },
}

pub struct BinaryOp<E> {
    pub op_type: BinaryOpType,
    pub lhs: Box<E>,
    pub rhs: Box<E>,
}

#[rustfmt::skip]
pub enum BinaryOpType {
    Add, Sub, Mul, Div, Rem, Shl, Shr, And, Or, Xor,
    Equal, NotEqual, Greater, GreaterOrEqual, Less, LessOrEqual,
    Identity, LogicAnd, LogicOr
}

pub struct UnaryOp<E> {
    pub op_type: UnaryOpType,
    pub expr: Box<E>,
}

#[rustfmt::skip]
pub enum UnaryOpType {
    Neg, Not, LogicNot, IntToReal, Floor, Ceil, Trunc, Round
}
