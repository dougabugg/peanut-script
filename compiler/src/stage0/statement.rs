use super::shared::{BufferSetSlice, If, IfElse, SharedStatement};
use super::{ops, CodeGenerator, Expr};

pub enum Statement {
    InitLocal(u8),
    Loop(Loop),
    Break { label: usize },
    Continue { label: usize },
    Other(SharedStatement<Statement, Expr>),
}

impl Statement {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            Statement::InitLocal(i) => g.push(ops::StackStore::new(i).into()),
            Statement::Loop(loop_) => loop_.compile(g),
            Statement::Break { label } => g.push_jump(*label, ops::Jump::new(0).into()),
            Statement::Continue { label } => g.push_jump(*label, ops::Jump::new(0).into()),
            Statement::Other(s) => s.compile(g),
        }
    }
}

pub struct Loop {
    condition: Option<Expr>,
    label_continue: Option<usize>,
    label_break: Option<usize>,
    body: Vec<Statement>,
}

impl Loop {
    pub fn compile(&self, g: &mut CodeGenerator) {
        let label_continue = match self.label_continue {
            Some(label_continue) => {
                g.register_label(label_continue);
                label_continue
            }
            None => g.create_label(),
        };
        let label_break = match self.label_break {
            Some(label_break) => {
                g.register_label(label_break);
                label_break
            }
            None => g.create_label(),
        };
        g.label_here(label_continue);
        if let Some(condition) = &self.condition {
            // compile condition expression
            condition.compile(g);
            // if zero, jump to label_break
            g.push_jump(label_break, ops::JumpZero::new(0).into());
        }
        // compile loop body
        for statement in &self.body {
            statement.compile(g);
        }
        // jump to label_continue
        g.push_jump(label_continue, ops::Jump::new(0).into());
        g.label_here(label_break);
    }
}

impl SharedStatement<Statement, Expr> {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            SharedStatement::Expr(e) => e.compile(g),
            SharedStatement::Return(e) => {
                e.compile(g);
                g.push(ops::Return.into());
            }
            SharedStatement::IfElse(f) => f.compile(g),
            SharedStatement::SeqAppend { seq, src } => {
                seq.compile(g);
                src.compile(g);
                g.push(ops::SeqAppend.into());
            }
            SharedStatement::SeqResize { seq, len } => {
                seq.compile(g);
                len.compile(g);
                g.push(ops::SeqResize.into());
            }
            SharedStatement::ListPush { list, value } => {
                list.compile(g);
                value.compile(g);
                g.push(ops::ListPush.into());
            }
            SharedStatement::BufferSetSlice(b) => b.compile(g),
        }
    }
}

impl IfElse<Statement, Expr> {
    pub fn compile(&self, g: &mut CodeGenerator) {
        let label_endif = g.create_label();
        // compile "if"
        self.if_.compile(g, label_endif);
        // compile "else if"'s
        for if_ in &self.else_if {
            if_.compile(g, label_endif);
        }
        // compile "else" body statements
        for statement in &self.else_ {
            statement.compile(g);
        }
        g.label_here(label_endif);
    }
}

impl If<Statement, Expr> {
    pub fn compile(&self, g: &mut CodeGenerator, label_endif: usize) {
        let label_next = g.create_label();
        // compile condition
        self.condition.compile(g);
        // if zero, jump to label_next
        g.push_jump(label_next, ops::JumpZero::new(0).into());
        // compile body statements
        for statement in &self.body {
            statement.compile(g);
        }
        // jump to label_endif
        g.push_jump(label_endif, ops::Jump::new(0).into());
        g.label_here(label_next);
    }
}

impl BufferSetSlice<Expr> {
    pub fn compile(&self, g: &mut CodeGenerator) {
        self.buffer.compile(g);
        self.src.compile(g);
        self.src_offset.compile(g);
        self.offset.compile(g);
        self.len.compile(g);
        g.push(ops::BufferSetSlice.into());
    }
}
