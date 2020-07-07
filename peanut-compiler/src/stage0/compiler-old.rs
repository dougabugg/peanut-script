struct Label {
    label_index: usize,
    op_index: usize,
    jump_indices: Vec<usize>,
}

struct ExprCursor<'a> {
    cursor: &'a Expr,
    parent: Option<&'a Expr>,
}

struct CompiledFunction {
    codegen: Vec<Op>,
    stack_index: u8,
    max_stack_index: u8,
    labels: Vec<Label>,
    locals: BTreeMap<usize, u8>,
}

impl FunctionCompiler {
    fn new() -> CompiledFunction {
        CompiledFunction {
            codegen: Vec::new(),
            stack_size: 0,
            max_stack_index: 0,
        }
    }

    fn compile(func: &Function) -> CompiledFunction {
        let mut cf = CompiledFunction::new();
        cf.stack_index = func.args.len();
        cf.max_stack_index = cf.stack_index;
    }

    fn compile_statement(&mut self, s: &Statement) {
        match s {
            Statement::Expr(e) => {

            },
            Statement::DeclareLocal(index) => {
                let stack_index = self.stack_push();
                self.locals.insert(index, stack_index);
            },
            Statement::DropLocal(i) => self.stack_pop(),
            Statement::Return(e) => {
                self.codegen.push()
            },
        }
    }

    fn stack_push(&mut self) -> u8 {
        if self.stack_size == u8::MAX {
            panic!("TODO / FIXME: ran out of stack space during codegen")
        }
        self.stack_index += 1;
        if self.stack_index > self.max_stack_index {
            self.max_stack_index = self.stack_index;
        }
        self.stack_index
    }

    fn stack_pop(&mut self) -> u8 {
        if self.stack_size == 0 {
            panic!("TODO / FIXME: attempted to pop an empty stack")
        }
        self.stack_index -= 1;
        self.stack_index
    }

    fn compile_expr(&mut self, e: &Expr) -> u8 {
        match e {
            Expr::Literal(lit) => {
                if let Literal::String(n) = lit {
                    // TODO
                    panic!();
                } else {
                    let lv = match lit {
                        Literal::None => LiteralValue::None,
                        Literal::Integer(i) => LiteralValue::Integer(i),
                        Literal::Real(r) => LiteralValue::Real(r),
                        _ => unreachable!(),
                    }
                    let out = self.stack_push();
                    self.codegen.push(LiteralCreate::new(lv, out).into());
                    out
                }
            },
            Expr::LocalScope(i) => {
                *self.locals.get(i)
            },
            Expr::ModuleScope(_) => {
                // TODO
                panic!();
            },
            Expr::Assign { place, value } => {
                match place {
                    Expr::LocalScope(i) => {

                    },
                    Expr::SeqIndex { seq, index } => {

                    },
                    Expr::SeqSlice { seq, a, b } => {

                    },
                    _ => {
                        // not a valid "place expression"
                        // TODO fix error handling?
                        panic!();
                    }
                }
            },
            Expr::BinaryOp { op, lhs, rhs } => {

                match op {
                    BinaryOp::Add => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Add::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Sub => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Sub::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Mul => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Mul::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Div => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Div::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Rem => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Rem::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Shl => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Shl::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Shr => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Shr::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::And => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(And::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Or => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Or::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Xor => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Xor::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::Equal => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: if output 0, return true
                        self.codegen.push(JumpZero::new(out, i + 4).into());
                        // i[2]: return false
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(0), out).into());
                        // i[3]:
                        self.codegen.push(Jump::new(i + 5).into());
                        // i[4]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[5]: next op
                        out
                    },
                    BinaryOp::NotEqual => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: if output 0, return false (output already false)
                        self.codegen.push(JumpZero::new(out, i + 3).into());
                        // i[2]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[3]: next op
                        out
                    },
                    BinaryOp::Greater => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: flip sign of out
                        self.codegen.push(Neg::new(out, out).into());
                        // i[2]: if neg, return true
                        self.codegen.push(JumpNeg::new(out, i + 5).into());
                        // i[3]: return false
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(0), out).into());
                        // i[4]:
                        self.codegen.push(Jump::new(i + 6).into());
                        // i[5]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[6]: next op
                        out
                    },
                    BinaryOp::GreaterOrEqual => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: if neg, return false
                        self.codegen.push(JumpNeg::new(out, i + 4).into());
                        // i[2]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[3]:
                        self.codegen.push(Jump::new(i + 5).into());
                        // i[4]: return false
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(0), out).into());
                        // i[5]: next op
                        out
                    },
                    BinaryOp::Less => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: if neg, return true
                        self.codegen.push(JumpNeg::new(out, i + 4).into());
                        // i[2]: return false
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(0), out).into());
                        // i[3]:
                        self.codegen.push(Jump::new(i + 5).into());
                        // i[4]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[5]: next op
                        out
                    },
                    BinaryOp::LessOrEqual => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        // i[0]: outputs -1 if a < b, 0 if a == b, 1 if a > b
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        // i[1]: flip sign of out
                        self.codegen.push(Neg::new(out, out).into());
                        // i[2]: if neg, return false
                        self.codegen.push(JumpNeg::new(out, i + 5).into());
                        // i[3]: return true
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(1), out).into());
                        // i[4]:
                        self.codegen.push(Jump::new(i + 6).into());
                        // i[5]: return false
                        self.codegen.push(LiteralCreate::new(LiteralValue::Integer(0), out).into());
                        // i[6]: next op
                        out
                    },
                    BinaryOp::Identity => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(Cmp::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::SameType => {
                        let lhs = self.compile_expr(lhs);
                        let rhs = self.compile_expr(rhs);
                        self.stack_pop();
                        let out = lhs;
                        self.codegen.push(SameType::new(lhs, rhs, out).into());
                        out
                    },
                    BinaryOp::LogicAnd => {
                        let i = self.codegen.len() as u32;
                        let lhs = self.compile_expr(lhs);
                        // i[0]: if 0 (false), return false early
                        self.codegen.push(JumpZero::new(out, i + 0).into());
                        // ... what is offset of next op?
                        // TODO we need to refactor this code, we can't implement LogicAnd/Or
                        // until we do
                    },
                }
            },
        }
    }
}
