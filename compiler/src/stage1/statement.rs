use super::shared::{SharedStatement, IfElse, If, BufferSetSlice};
use super::stage0;

pub enum Statement {
    BindLocal(usize),
    DropLocal(usize),
    Loop(Loop),
    Break { label: Option<usize> },
    Continue { label: Option<usize> },
    Other(SharedStatement<Statement, Expr>),
}

impl Statement {
    pub fn convert(self) -> stage0::Statement {
        panic!();
    }
}

pub struct Loop {
    condition: Option<Expr>,
    label: Option<usize>,
    body: Vec<Statement>,
}

impl Loop {
    pub fn convert(self) -> stage0::Loop {
        panic!();
    }
}

impl SharedStatement<Statement, Expr> {
    pub fn convert(self) -> SharedStatement<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl IfElse<Statement, Expr> {
    pub fn convert(self) -> IfElse<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl If<Statement, Expr> {
    pub fn convert(self) -> If<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl BufferSetSlice<Statement, Expr> {
    pub fn convert(self) -> BufferSetSlice<stage0::Statement, stage0::Expr> {
        panic!();
    }
}
