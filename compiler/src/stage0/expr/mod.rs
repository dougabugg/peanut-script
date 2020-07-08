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
    Assign { place: Box<Expr>, value: Box<Expr> },
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
            Expr::Assign { place, value } => {
                match **place {
                    Expr::LocalScope(i) => {
                        g.append(value.compile());
                        g.push(ops::StackStore::new(i).into());
                    }
                    Expr::ModuleScope(i) => {
                        g.push(ops::StackLoad::new(0).into());
                        g.push(ops::LiteralCreate::new((i as i64).into()).into());
                        g.append(value.compile());
                        g.push(ops::SeqSet.into());
                    }
                    // TODO more here ... => { ... },
                    _ => panic!("invalid place expression"),
                }
            }
        }
        g.into_vec()
    }
}
