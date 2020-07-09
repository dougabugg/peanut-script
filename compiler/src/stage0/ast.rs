pub enum Statement {
    Expr(Expr),
    BindLocal(u8),
    DropLocal(u8),
    Return(Expr),
    IfElse(IfElse),
    Loop {
        label: Option<usize>,
        body: Vec<Statement>,
    },
    Break {
        label: Option<usize>,
    },
    Continue {
        label: Option<usize>,
    },
    SeqAppend {
        seq: Expr,
        src: Expr,
    },
    SeqResize {
        seq: Expr,
        size: Expr,
    },
    ListPush {
        list: Expr,
        value: Expr,
    },
    BufferSetSlice {
        buffer: Expr,
        src: Expr,
        src_offset: Expr,
        offset: Expr,
        len: Expr,
    },
}

pub struct IfElse {
    if_: If,
    else_if: Vec<If>,
    else_: Vec<Statement>,
}

pub struct If {
    condition: Expr,
    body: Vec<Statement>,
}

pub struct Function {
    args: Vec<usize>,
    body: Vec<Statement>,
}

pub struct Module {
    items: Vec<ModuleItem>,
}

pub enum ModuleItem {
    ModuleRef(usize),
    Function(Function),
    Literal(Literal),
}

pub struct Program {
    modules: Vec<Module>,
}
