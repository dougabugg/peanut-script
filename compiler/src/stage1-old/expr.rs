use super::shared::{SharedExpr, BinaryOp, UnaryOp};
use super::stage0;

pub struct Expr {
    pub span: (),
    pub inner: ExprInner,
}

impl Expr {
    pub fn find_locals(&self) -> Vec<usize> {
        match self.inner {
            ExprInner::LocalScope(i) => vec![i],
            ExprInner::Other(s) => s.find_locals(),
        }
    }
}

pub enum ExprInner {
    LocalScope(usize),
    Other(SharedExpr<Expr>),
}

impl SharedExpr<Expr> {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        match self {
            SharedExpr::BinaryOp(b) => {
                v.append(&mut b.lhs.find_locals());
                v.append(&mut b.rhs.find_locals());
            }
            SharedExpr::UnaryOp(u) => v.append(&mut u.expr.find_locals()),
            SharedExpr::Call { function, args } => {
                v.append(&mut function.find_locals());
                for arg in args {
                    v.append(&mut arg.find_locals());
                }
            }
            SharedExpr::Assign { place, value } => {
                v.append(&mut place.find_locals());
                v.append(&mut value.find_locals());
            }
            SharedExpr::SeqIndex { seq, index } => {
                v.append(&mut seq.find_locals());
                v.append(&mut index.find_locals());
            }
            SharedExpr::SeqLen { seq } => v.append(&mut seq.find_locals()),
            SharedExpr::SeqToList { seq } => v.append(&mut seq.find_locals()),
            SharedExpr::TupleCreate(items) => {
                for item in items {
                    v.append(&mut item.find_locals());
                }
            }
            SharedExpr::TupleFromList(list) => v.append(&mut list.find_locals()),
            SharedExpr::TableCreate(list) => v.append(&mut list.find_locals()),
            SharedExpr::ListCreate(items) => {
                for item in items {
                    v.append(&mut item.find_locals());
                }
            }
            SharedExpr::ListGetSlice { list, a, b } => {
                v.append(&mut list.find_locals());
                v.append(&mut a.find_locals());
                v.append(&mut b.find_locals());
            }
            SharedExpr::ListPop(list) => v.append(&mut list.find_locals()),
            SharedExpr::BufferCreate(size) => v.append(&mut size.find_locals()),
            SharedExpr::BufferGetSlice { buffer, a, b } => {
                v.append(&mut buffer.find_locals());
                v.append(&mut a.find_locals());
                v.append(&mut b.find_locals());
            }
        }
        v
    }

    pub fn convert(self) -> SharedExpr<stage0::Expr> {
        match self {
            SharedExpr::LiteralValue(l) => SharedExpr::LiteralValue(l),
            SharedExpr::ModuleScope(i) => SharedExpr::ModuleScope(i),
            SharedExpr::BinaryOp => SharedExpr::BinaryOp,
            SharedExpr::UnaryOp => SharedExpr::UnaryOp,
            SharedExpr::Call => SharedExpr::Call,
            SharedExpr::Assign => SharedExpr::Assign,
            SharedExpr::SeqIndex => SharedExpr::SeqIndex,
            SharedExpr::SeqLen => SharedExpr::SeqLen,
            SharedExpr::SeqToList => SharedExpr::SeqToList,
            SharedExpr::TupleCreate => SharedExpr::TupleCreate,
            SharedExpr::TupleFromList => SharedExpr::TupleFromList,
            SharedExpr::TableCreate => SharedExpr::TableCreate,
            SharedExpr::ListCreate => SharedExpr::ListCreate,
            SharedExpr::ListGetSlice => SharedExpr::ListGetSlice,
            SharedExpr::ListPop => SharedExpr::ListPop,
            SharedExpr::BufferCreate => SharedExpr::BufferCreate,
            SharedExpr::BufferGetSlice => SharedExpr::BufferGetSlice,
        }
    }
}

impl BinaryOp<Expr> {
    pub fn convert(self) -> BinaryOp<stage0::Expr> {
        panic!();
    }
}

impl UnaryOp<Expr> {
    pub fn convert(self) -> BinaryOp<stage0::Expr> {
        panic!();
    }
}
