use super::shared::{SharedStatement, IfElse, If, BufferSetSlice};
use super::stage0;

pub enum Statement {
    BindLocal(usize),
    DropLocal(usize),
    InitLocal(usize),
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
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        if let Some(e) = self.condition {
            v.append(&mut e.find_locals());
        }
        for statement in &self.body {
            v.append(&mut statement.find_locals());
        }
        v
    }

    pub fn convert(self) -> stage0::Loop {
        panic!();
    }
}

impl SharedStatement<Statement, Expr> {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        match self {
            SharedStatement::Expr(e) => v.append(&mut e.find_locals()),
            SharedStatement::Return(e) => v.append(&mut e.find_locals()),
            SharedStatement::IfElse(i) => v.append(&mut i.find_locals()),
            SharedStatement::SeqAppend { seq, src } => {
                v.append(&mut seq.find_locals());
                v.append(&mut src.find_locals());
            }
            SharedStatement::SeqResize { seq, len } => {
                v.append(&mut seq.find_locals());
                v.append(&mut len.find_locals());
            }
            SharedStatement::ListPush { list, value } => {
                v.append(&mut list.find_locals());
                v.append(&mut value.find_locals());
            }
            SharedStatement::BufferSetSlice(b) => v.append(&mut b.find_locals())
        }
        v
    }

    pub fn convert(self) -> SharedStatement<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl IfElse<Statement, Expr> {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        v.append(&mut self.if_.find_locals());
        for i in &self.else_if {
            v.append(&mut i.find_locals());
        }
        for s in &self.else_ {
            v.append(&mut s.find_locals());
        }
        v
    }

    pub fn convert(self) -> IfElse<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl If<Statement, Expr> {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        v.append(&mut self.condition.find_locals());
        for s in &self.body {
            v.append(&mut s.find_locals());
        }
        v
    }

    pub fn convert(self) -> If<stage0::Statement, stage0::Expr> {
        panic!();
    }
}

impl BufferSetSlice<Statement, Expr> {
    pub fn find_locals(&self) -> Vec<usize> {
        let mut v = Vec::new();
        v.append(&mut buffer.find_locals());
        v.append(&mut src.find_locals());
        v.append(&mut src_offset.find_locals());
        v.append(&mut offset.find_locals());
        v.append(&mut len.find_locals());
        v
    }

    pub fn convert(self) -> BufferSetSlice<stage0::Statement, stage0::Expr> {
        panic!();
    }
}
