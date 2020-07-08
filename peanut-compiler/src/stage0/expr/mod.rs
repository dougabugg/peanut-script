use super::{CodeGenerator, Compile, Op, ops};

mod binaryop;
mod unaryop;

pub use binaryop::{BinaryOp, BinaryOpType};
pub use unaryop::{UnaryOp, UnaryOpType};

pub enum Literal {
    None,
    Integer(i64),
    Real(f64),
    // String(usize),
}

pub enum Expr {
    Literal(Literal),
    LocalScope(u8),
    ModuleScope(usize),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Assign {
        place: Box<Expr>,
        value: Box<Expr>,
    },
}

impl Compile for Expr {
    fn compile(&self) -> Vec<Op> {
        let mut g = CodeGenerator::new();
        match self {
            Expr::Literal(l) => panic!("TODO"),
            Expr::LocalScope(i) => g.push(ops::StackLoad::new(i)),
            Expr::ModuleScope(i) => {
                g.push(ops::StackLoad::new(0));
                g.push(ops::LiteralCreate::new(i as i64).into());
                g.push(ops::SeqGet.into());
            },
            Expr::BinaryOp(b) => return b.compile(),
            Expr::UnaryOp(u) => return u.compile(),
            Expr::Assign { place, value } => {
                match place {
                    Expr::LocalScope(i) => {
                        g.append(value.compile());
                        g.push(ops::StackStore::new(i));
                    },
                    Expr::ModuleScope(i) => {
                        g.push(ops::StackLoad::new(0));
                        g.push(ops::LiteralCreate::new(i as i64).into());
                        g.append(value.compile());
                        g.push(ops::SeqSet.into());
                    },
                    _ => panic!("invalid place expression"),
                }
            },
        }
        g.into_vec()
    }
}
