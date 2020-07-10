use super::shared::SharedExpr;
use super::{ops, CodeGenerator};

pub struct Expr {
    pub span: (),
    pub inner: ExprInner,
}

impl Expr {
    pub fn compile(&self, g: &mut CodeGenerator) {
        // TODO handle source mapping (span field) here
        match &self.inner {
            ExprInner::LocalScope(i) => g.push(ops::StackLoad::new(*i).into()),
            ExprInner::Other(s) => s.compile(g),
        }
    }
}

pub enum ExprInner {
    LocalScope(u8),
    Other(SharedExpr<Expr>),
}

impl SharedExpr<Expr> {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            SharedExpr::LiteralValue(l) => g.push(ops::LiteralCreate::new(*l).into()),
            SharedExpr::ModuleScope(i) => {
                g.push(ops::StackLoad::new(0).into());
                g.push(ops::LiteralCreate::new((*i as i64).into()).into());
                g.push(ops::SeqGet.into());
            }
            SharedExpr::BinaryOp(b) => b.compile(g),
            SharedExpr::UnaryOp(u) => u.compile(g),
            SharedExpr::Call { function, args } => {
                function.compile(g);
                assert!(args.len() <= 255);
                for arg in args {
                    arg.compile(g);
                }
                g.push(ops::Call::new(args.len() as u8).into());
            }
            SharedExpr::Assign { place, value } => match &place.inner {
                ExprInner::LocalScope(i) => {
                    value.compile(g);
                    g.push(ops::StackStore::new(*i).into());
                }
                ExprInner::Other(s) => match s {
                    SharedExpr::ModuleScope(i) => {
                        g.push(ops::StackLoad::new(0).into());
                        g.push(ops::LiteralCreate::new((*i as i64).into()).into());
                        value.compile(g);
                        g.push(ops::SeqSet.into());
                    }
                    SharedExpr::SeqIndex { seq, index } => {
                        seq.compile(g);
                        index.compile(g);
                        value.compile(g);
                        g.push(ops::SeqSet.into());
                    }
                    _ => panic!("invalid place expression"),
                },
            },
            SharedExpr::SeqIndex { seq, index } => {
                seq.compile(g);
                index.compile(g);
                g.push(ops::SeqGet.into());
            }
            SharedExpr::SeqLen { seq } => {
                seq.compile(g);
                g.push(ops::SeqLen.into());
            }
            SharedExpr::SeqToList { seq } => {
                seq.compile(g);
                g.push(ops::SeqToList.into());
            }
            SharedExpr::TupleCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    item.compile(g);
                }
                g.push(ops::TupleCreate::new(items.len() as u8).into());
            }
            SharedExpr::TupleFromList(list) => {
                list.compile(g);
                g.push(ops::TupleFromList.into());
            }
            SharedExpr::TableCreate(list) => {
                list.compile(g);
                g.push(ops::TableCreate.into());
            }
            SharedExpr::ListCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    item.compile(g);
                }
                g.push(ops::ListCreate::new(items.len() as u8).into());
            }
            SharedExpr::ListGetSlice { list, a, b } => {
                list.compile(g);
                a.compile(g);
                b.compile(g);
                g.push(ops::ListGetSlice.into());
            }
            SharedExpr::ListPop(list) => {
                list.compile(g);
                g.push(ops::ListPop.into());
            }
            SharedExpr::BufferCreate(size) => {
                size.compile(g);
                g.push(ops::BufferCreate.into());
            }
            SharedExpr::BufferGetSlice { buffer, a, b } => {
                buffer.compile(g);
                a.compile(g);
                b.compile(g);
                g.push(ops::BufferGetSlice.into());
            }
        }
    }
}
