pub struct Function {
    pub args: Vec<usize>,
    pub body: Vec<Statement>,
}

struct DeferredDrop {
    loc: usize,
    var: Span<Var>,
}

struct BlockScopeAnalysis<'a> {
    seen: &'a mut BTreeSet<Var>,
    bindings: BTreeSet<Span<Var>>,
    drops: Vec<DeferredDrop>,
    loc: usize,
}

impl<'a> BlockScopeAnalysis<'a> {
    pub fn new(seen: &'a mut BTreeSet<Var>) -> Self {
        BlockScopeAnalysis {
            seen,
            bindings: BTreeSet::new(),
            drops: Vec::new(),
            loc: 0,
        }
    }

    pub fn process_var(&mut self, var: Span<Var>) {
        let dropped = self.seen.insert(var.inner);
        if dropped {
            self.drops.push(DeferredDrop { self.loc, var });
        }
    }

    pub fn process_expr(&mut self, expr: &Expr) {
        for var in expr.find_vars() {
            self.process_var(var);
        }
    }

    pub fn process_child_block(&mut self, block: &mut Vec<Statement>) {
        let mut b = BlockScopeAnalysis::new(&mut self.seen);
        let outer_scope_vars = b.process_block(block);
        for var in outer_scope_vars {
            self.drops.push(DeferredDrop { self.loc, var });
        }
    }

    pub fn process_if(&mut self, if_: &mut If) {
        self.process_expr(&if_.condition);
        self.process_child_block(&mut if_.body);
    }

    pub fn process_block(&mut self, block: &mut Vec<Statement>) -> Vec<Span<Var>> {
        for (loc, statement) in block.iter_mut().enumerate().rev() {
            self.loc = loc;
            match statement {
                Statement::BindLocal(i) => self.bindings.insert(i),
                Statement::DropLocal(i) => {
                    panic!("unexpected DropLocal statement during block scope analysis");
                },
                Statement::InitLocal(i) => self.process_var(i),
                Statement::Loop(l) => {
                    self.process_expr(&l.condition);
                    self.process_child_block(&mut l.body);
                },
                Statement::Break { label } => {},
                Statement::Continue { label } => {},
                Statement::Expr(expr) => self.process_expr(expr),
                Statement::Return(expr) => self.process_expr(expr),
                Statement::IfElse(s) => {
                    self.process_if(s.if_);
                    for if_ in s.else_if {
                        self.process_if(if_);
                    }
                    self.process_child_block(&mut s.else_);
                },
                Statement::Assign { place, value } => {
                    self.process_expr(place);
                    self.process_expr(value);
                },
                Statement::SeqAppend { seq, src } => {
                    self.process_expr(seq);
                    self.process_expr(src);
                },
                Statement::SeqResize { seq, len } => {
                    self.process_expr(seq);
                    self.process_expr(len);
                },
                Statement::ListPush { list, value } => {
                    self.process_expr(list);
                    self.process_expr(value);
                },
                Statement::BufferSetSlice(b) => {
                    self.process_expr(&b.buffer);
                    self.process_expr(&b.src);
                    self.process_expr(&b.src_offset);
                    self.process_expr(&b.offset);
                    self.process_expr(&b.len);
                },
            }
        }
    }
}