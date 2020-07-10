pub struct Function {
    pub args: Vec<Var>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn compile(self) -> bytecode::Function {
        let mut setup = Vec::new();
        for arg in self.args {
            setup.push(Statement::BindVar(arg));
            setup.push(Statement::InitVar(arg));
        }
        setup.append(&mut self.body);
        self.body = setup;
        if let Err(unknown_scope_vars) = self.block_scope_analysis() {
            panic!("found {} variables with unknown scope", unknown_scope_vars.len());
        }
        let mut g = CodeGenerator::new();
        for statement in &self.body {
            statement.compile(&mut g);
        }
        bytecode::Function {
            ops: g.into_vec()
        }
    }

    fn block_scope_analysis(&mut self) -> Result<(), Vec<Span<Var>>> {
        let mut seen = BTreeSet::new();
        let mut b = BlockScopeAnalysis::new(&mut seen);
        let unknown_scope_vars = b.process_block(&mut self.body);
        if unknown_scope_vars.is_empty() {
            Ok(())
        } else {
            Err(unknown_scope_vars)
        }
    }
}

struct DeferredDrop {
    loc: usize,
    var: Span<Var>,
}

struct BlockScopeAnalysis<'a> {
    seen: &'a mut BTreeSet<Var>,
    bindings: BTreeSet<Var>,
    drops: Vec<DeferredDrop>,
    loc: usize,
}

impl<'a> BlockScopeAnalysis<'a> {
    fn new(seen: &'a mut BTreeSet<Var>) -> Self {
        BlockScopeAnalysis {
            seen,
            bindings: BTreeSet::new(),
            drops: Vec::new(),
            loc: 0,
        }
    }

    fn process_block(&mut self, block: &mut Vec<Statement>) -> Vec<Span<Var>> {
        for (loc, statement) in block.iter_mut().enumerate().rev() {
            self.loc = loc;
            self.process_statement(statement);
        }
        let mut parent_scope = Vec::new();
        for drop in self.drops.iter().rec() {
            if bindings.contain(&drop.var.inner) {
                block.insert(drop.loc + 1, Statement::DropLocal(drop.var.inner));
            } else {
                parent_scope.push(drop.var);
            }
        }
        parent_scope
    }

    fn process_var(&mut self, var: Span<Var>) {
        let dropped = self.seen.insert(var.inner);
        if dropped {
            self.drops.push(DeferredDrop { self.loc, var });
        }
    }

    fn process_expr(&mut self, expr: &Expr) {
        for var in expr.find_vars() {
            self.process_var(var);
        }
    }

    fn process_child_block(&mut self, block: &mut Vec<Statement>) {
        let mut b = BlockScopeAnalysis::new(&mut self.seen);
        let outer_scope_vars = b.process_block(block);
        for var in outer_scope_vars {
            self.drops.push(DeferredDrop { self.loc, var });
        }
    }

    fn process_if(&mut self, if_: &mut If) {
        self.process_expr(&if_.condition);
        self.process_child_block(&mut if_.body);
    }

    fn process_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::BindVarl(i) => self.bindings.insert(i.inner),
            Statement::DropVar(i) => {
                panic!("unexpected DropVar statement during block scope analysis");
            },
            Statement::InitVar(i) => self.process_var(i),
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
