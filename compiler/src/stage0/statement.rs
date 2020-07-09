use super::{ops, CodeGenerator, Expr, Op};

pub enum Statement {
    Expr(Expr),
    // BindLocal(u8),
    // DropLocal(u8),
    Return(Expr),
    IfElse(IfElse),
    Loop(Loop),
    Break {
        label: usize,
    },
    Continue {
        label: usize,
    },
    SeqAppend {
        seq: Expr,
        src: Expr,
    },
    SeqResize {
        seq: Expr,
        len: Expr,
    },
    ListPush {
        list: Expr,
        value: Expr,
    },
    BufferSetSlice(Box<BufferSetSlice>),
}

impl Statement {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            Statement::Expr(e) => g.append(e.compile()),
            Statement::Return(e) => {
                g.append(e.compile());
                g.push(ops::Return.into());
            }
            Statement::IfElse(f) => f.compile(g),
            Statement::Loop(loop_) => loop_.compile(g),
            Statement::Break { label } => g.push_jump(*label, ops::Jump::new(0).into()),
            Statement::Continue { label } => g.push_jump(*label, ops::Jump::new(0).into()),
            Statement::SeqAppend { seq, src } => {
                g.append(seq.compile());
                g.append(src.compile());
                g.push(ops::SeqAppend.into());
            }
            Statement::SeqResize { seq, len } => {
                g.append(seq.compile());
                g.append(len.compile());
                g.push(ops::SeqResize.into());
            }
            Statement::ListPush { list, value } => {
                g.append(list.compile());
                g.append(value.compile());
                g.push(ops::ListPush.into());
            }
            Statement::BufferSetSlice(b) => g.append(b.compile()),
        }
    }
}

pub struct IfElse {
    if_: If,
    else_if: Vec<If>,
    else_: Vec<Statement>,
}

impl IfElse {
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

pub struct If {
    condition: Expr,
    body: Vec<Statement>,
}

impl If {
    pub fn compile(&self, g: &mut CodeGenerator, label_endif: usize) {
        let label_next = g.create_label();
        // compile condition
        g.append(self.condition.compile());
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

pub struct Loop {
    loop_type: LoopType,
    label_continue: Option<usize>,
    label_break: Option<usize>,
    body: Vec<Statement>,
}

pub enum LoopType {
    Infinite,
    While(Expr),
    DoWhile(Expr),
}

impl Loop {
    pub fn compile(&self, g: &mut CodeGenerator) {
        let label_continue = match self.label_continue {
            Some(label_continue) => {
                g.register_label(label_continue);
                label_continue
            }
            None => g.create_label()
        };
        let label_break = match self.label_break {
            Some(label_break) => {
                g.register_label(label_break);
                label_break
            }
            None => g.create_label()
        };
        g.label_here(label_continue);
        if let LoopType::While(condition) = &self.loop_type {
            // compile condition expression
            g.append(condition.compile());
            // if zero, jump to label_break
            g.push_jump(label_break, ops::JumpZero::new(0).into());
        }
        // compile loop body
        for statement in &self.body {
            statement.compile(g);
        }
        if let LoopType::DoWhile(condition) = &self.loop_type {
            // compile condition expression
            g.append(condition.compile());
            // if zero, jump to label_break
            g.push_jump(label_break, ops::JumpZero::new(0).into());
        }
        // jump to label_continue
        g.push_jump(label_continue, ops::Jump::new(0).into());
        g.label_here(label_break);
    }
}

pub struct BufferSetSlice {
    buffer: Expr,
    src: Expr,
    src_offset: Expr,
    offset: Expr,
    len: Expr,
}

impl BufferSetSlice {
    pub fn compile(&self) -> Vec<Op> {
        let mut g = CodeGenerator::new();
        g.append(self.buffer.compile());
        g.append(self.src.compile());
        g.append(self.src_offset.compile());
        g.append(self.offset.compile());
        g.append(self.len.compile());
        g.push(ops::BufferSetSlice.into());
        g.into_vec()
    }
}
