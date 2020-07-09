use super::shared::{SharedExpr, BinaryOp, UnaryOp};
use super::stage0;

pub struct Expr {
    pub span: (),
    pub inner: ExprInner,
}

impl Expr {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        match self.inner {
            ExprInner::LocalScope(i) => v.push(i),
            ExprInner::Other(s) => v.append(&mut s.find_locals()),
        }
        v
    }
}

pub enum ExprInner {
    LocalScope(usize),
    Other(SharedExpr<Expr>),
}

// TODO
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
            },
            SharedExpr::Assign { place, value } => {
                
            },
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
