use super::{ops, CodeGenerator, Expr, Op};

pub struct UnaryOp {
    op_type: UnaryOpType,
    expr: Box<Expr>,
}

#[rustfmt::skip]
pub enum UnaryOpType {
    Neg, Not, LogicNot, IntToReal, Floor, Ceil, Trunc, Round
}

impl UnaryOp {
    pub fn compile(&self) -> Vec<Op> {
        let mut g = CodeGenerator::new();
        g.append(self.expr.compile());
        match self.op_type {
            UnaryOpType::Neg => {
                g.push(ops::Neg.into());
            }
            UnaryOpType::Not => {
                g.push(ops::Not.into());
            }
            UnaryOpType::LogicNot => {
                let label_true = g.create_label();
                let label_next = g.create_label();
                // if 0, jump to label_true
                g.push_jump(label_true, ops::JumpZero::new(0).into());
                // push false (0)
                g.push(ops::LiteralCreate::new(0.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push true (1)
                g.label_here(label_true);
                g.push(ops::LiteralCreate::new(1.into()).into());
                g.label_here(label_next);
            }
            UnaryOpType::IntToReal => {
                g.push(ops::IntToReal.into());
            }
            UnaryOpType::Floor => {
                g.push(ops::Floor.into());
            }
            UnaryOpType::Ceil => {
                g.push(ops::Ceil.into());
            }
            UnaryOpType::Trunc => {
                g.push(ops::Trunc.into());
            }
            UnaryOpType::Round => {
                g.push(ops::Round.into());
            }
        }
        g.into_vec()
    }
}
