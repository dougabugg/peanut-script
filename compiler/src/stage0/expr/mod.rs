use super::{ops, CodeGenerator, Op};

mod binaryop;
mod unaryop;

pub use binaryop::{BinaryOp, BinaryOpType};
pub use unaryop::{UnaryOp, UnaryOpType};

pub type Literal = ops::LiteralValue;

pub enum Expr {
    Literal(Literal),
    LocalScope(u8),
    ModuleScope(usize),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Call {
        function: Box<Expr>,
        args: Vec<Expr>,
    },
    Assign {
        place: Box<Expr>,
        value: Box<Expr>,
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
    pub fn compile(&self) -> Vec<Op> {
        let mut g = CodeGenerator::new();
        match self {
            Expr::Literal(l) => g.push(ops::LiteralCreate::new(*l).into()),
            Expr::LocalScope(i) => g.push(ops::StackLoad::new(*i).into()),
            Expr::ModuleScope(i) => {
                g.push(ops::StackLoad::new(0).into());
                g.push(ops::LiteralCreate::new((*i as i64).into()).into());
                g.push(ops::SeqGet.into());
            }
            Expr::BinaryOp(b) => return b.compile(),
            Expr::UnaryOp(u) => return u.compile(),
            Expr::Call { function, args } => {
                g.append(function.compile());
                assert!(args.len() <= 255);
                for arg in args {
                    g.append(arg.compile());
                }
                g.push(ops::Call::new(args.len() as u8).into());
            }
            Expr::Assign { place, value } => match &**place {
                Expr::LocalScope(i) => {
                    g.append(value.compile());
                    g.push(ops::StackStore::new(*i).into());
                }
                Expr::ModuleScope(i) => {
                    g.push(ops::StackLoad::new(0).into());
                    g.push(ops::LiteralCreate::new((*i as i64).into()).into());
                    g.append(value.compile());
                    g.push(ops::SeqSet.into());
                }
                Expr::SeqIndex { seq, index } => {
                    g.append(seq.compile());
                    g.append(index.compile());
                    g.append(value.compile());
                    g.push(ops::SeqSet.into());
                }
                _ => panic!("invalid place expression"),
            },
            Expr::SeqIndex { seq, index } => {
                g.append(seq.compile());
                g.append(index.compile());
                g.push(ops::SeqGet.into());
            }
            Expr::SeqLen { seq } => {
                g.append(seq.compile());
                g.push(ops::SeqLen.into());
            }
            Expr::SeqToList { seq } => {
                g.append(seq.compile());
                g.push(ops::SeqToList.into());
            }
            Expr::TupleCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    g.append(item.compile());
                }
                g.push(ops::TupleCreate::new(items.len() as u8).into());
            }
            Expr::TupleFromList(list) => {
                g.append(list.compile());
                g.push(ops::TupleFromList.into());
            }
            Expr::TableCreate(list) => {
                g.append(list.compile());
                g.push(ops::TableCreate.into());
            }
            Expr::ListCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    g.append(item.compile());
                }
                g.push(ops::ListCreate::new(items.len() as u8).into());
            }
            Expr::ListGetSlice { list, a, b } => {
                g.append(list.compile());
                g.append(a.compile());
                g.append(b.compile());
                g.push(ops::ListGetSlice.into());
            }
            Expr::ListPop(list) => {
                g.append(list.compile());
                g.push(ops::ListPop.into());
            }
            Expr::BufferCreate(size) => {
                g.append(size.compile());
                g.push(ops::BufferCreate.into());
            }
            Expr::BufferGetSlice { buffer, a, b } => {
                g.append(buffer.compile());
                g.append(a.compile());
                g.append(b.compile());
                g.push(ops::BufferGetSlice.into());
            }
        }
        g.into_vec()
    }
}
