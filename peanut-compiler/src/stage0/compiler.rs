use super::CodeGenerator;

struct CompiledFunction {
    codegen: CodeGenerator,
    stack_index: u8,
    max_stack_index: u8,
}

impl CompiledFunction {
    pub fn new() -> CompiledFunction {
        CompiledFunction {
            codegen: CodeGenerator::new(),
            stack_index: 0,
            max_stack_index: 0,
        }
    }

    fn stack_push(&mut self) -> u8 {
        let t = self.stack_index;
        self.stack_index += 1;
        if self.stack_index > self.max_stack_index {
            self.max_stack_index = self.stack_index;
        }
        t
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_index -= 1;
        self.stack_index
    }

    fn compile_expr(&mut self, e: &Expr) -> u8 {
        match e {
            Expr::Literal(literal) => self.compile_expr_literal(literal),
            Expr::LocalScope(i) => i as u8,
            Expr::ModuleScope(i) => {
                // TODO also take a look at LocalScope
                panic!();
            },
            Expr::Assign { place, value } => {
                // TODO
                panic!();
            },
            Expr::BinaryOp { op, lhs, rhs } => self.compile_binary_op(op, &lhs, &rhs),
            Expr::UnaryOp { op, expr } => {
                match op {
                    UnaryOp::LogicOr => {
                        let label_true = self.codegen.create_label();
                        let label_next = self.codegen.create_label();
                        let expr = self.compile_expr(expr);
                        let out = self.stack_push();
                        // if out is zero, jump to label_true
                        self.codegen.push_jump(label_true, JumpZero::new(expr, 0).into());
                        // set out to false (0)
                        self.codegen.push(LiteralCreate::new(0.into(), out).into());
                        // jump to label_next
                        self.codegen.push_jump(label_next, Jump::new(0).into());
                        // set out to true (1)
                        self.codegen.label_here(label_true);
                        self.codegen.push(LiteralCreate::new(1.into(), out).into());
                        self.codegen.label_here(label_next);
                        self.stack_pop();
                        out
                    },
                    _ => {
                        let expr = self.compile_expr(expr);
                        let out = self.stack_push();
                        self.codegen.push(match op {
                            UnaryOp::Neg => Neg::new(expr, out).into(),
                            UnaryOp::Not => Not::new(expr, out).into(),
                            UnaryOp::Floor => Floor::new(expr, out).into(),
                            UnaryOp::Ceil => Ceil::new(expr, out).into(),
                            UnaryOp::Trunc => Trunc::new(expr, out).into(),
                            UnaryOp::Round => Round::new(expr, out).into(),
                            UnaryOp::LogicOr => unreachable!(),
                        });
                        self.stack_pop();
                        out
                    }
                }
            },
            Expr::SeqIndex { seq, index } => {
                let seq = self.compile_expr(seq);
                let out = self.stack_push();
                let index = self.compile_expr(index);
                self.codegen.push(SeqGet::new(seq, index, out));
                self.stack_pop();
                out
            },
            Expr::SeqSlice { seq, a, b } => {
                let seq = self.compile_expr(seq);
                let out = self.stack_push();
                let a = self.compile_expr(a);
                self.stack_push();
                let b = self.compile_expr(b);
                self.stack_pop();
                // TODO create the SeqSlice operation in peanut-vm
                self.codegen.push(SeqSlice::new(seq, a, b, out));
                self.stack_pop();
                out
            },
            Expr::SeqResize { seq, size } => {
                let seq = self.compile_expr(seq);
                self.stack_push();
                let size = self.compile_expr(size);
                self.stack_pop();
                self.codegen.push(SeqResize::new(seq, size));
            },
            Expr::SeqAppend { seq, src } => {
                let seq = self.compile_expr(seq);
                self.stack_push();
                let src = self.compile_expr(src);
                self.stack_pop();
                self.codegen.push(SeqAppend::new(seq, src));
            },
            Expr::SeqToList(seq) => {
                let seq = self.compile_expr(seq);
                let out = self.stack_push();
                self.codegen.push(SeqToList::new(seq, out));
                self.stack_pop();
                out
            },
            Expr::Call { func, args } => {
                let func = self.compile_expr(func);
                let out = self.stack_push();
                let mut args2 = Vec::new();
                for arg in args {
                    let a = self.compile_expr(arg);
                    args2.push(a);
                    self.stack_push();
                    // TODO left off here
/*
we have a lot of refactoring to do. Expr::Call and others with Vec<Expr> need to be handled carefully,
since if vec.len() > u8::MAX, we need to create a list, push each item, then call *FromList, or
in the case of Call, panic since the bytecode doesn't support calling with args as a List.

also, I don't like the current API for this, the usage of stack_push() and stack_pop() are confusing
and inconsistent, and in a few spots leads to redundant copying and reserving space on the stack that
isn't used.

this current method doesn't work unless the AST is further processed, specifically: if there are more
than u8::MAX variables in LocalScope, we need to modify the AST to store excess variables in a tuple,
and if ... I can't remember the other exceptional case, but I think there was another issue that can be
mitigated by processed the AST. it might have been the "ordering" of variables, and inserting DropLocal
statements when variables aren't being used.

since the usage of stack_push/pop is so messy, I was thinking about switching over VM ops to use
arguments in a true runtime stack, not the "register stack" we have now. this would make code generation
some what easier, and might bring other benefits. this is what other virtual machine implementations
use (pretty sure python does it that way).
*/
                }
            },
        }
    }

    fn compile_binary_op(&mut self, op: BinaryOp, lhs: &Expr, rhs: &Expr) -> u8 {
        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem
            | BinaryOp::Shl | BinaryOp::Shr | BinaryOp::And | BinaryOp::Or | BinaryOp::Xor
            | BinaryOp::Identity | BinaryOp::SameType => {
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                self.codegen.push(match op {
                    BinaryOp::Add => Add::new(lhs, rhs, out).into(),
                    BinaryOp::Sub => Sub::new(lhs, rhs, out).into(),
                    BinaryOp::Mul => Mul::new(lhs, rhs, out).into(),
                    BinaryOp::Div => Div::new(lhs, rhs, out).into(),
                    BinaryOp::Rem => Rem::new(lhs, rhs, out).into(),
                    BinaryOp::Shl => Shl::new(lhs, rhs, out).into(),
                    BinaryOp::Shr => Shr::new(lhs, rhs, out).into(),
                    BinaryOp::And => And::new(lhs, rhs, out).into(),
                    BinaryOp::Or => Or::new(lhs, rhs, out).into(),
                    BinaryOp::Xor => Xor::new(lhs, rhs, out).into(),
                    BinaryOp::Identity => Cmp::new(lhs, rhs, out).into(),
                    BinaryOp::SameType => SameType::new(lhs, rhs, out).into(),
                    _ => unreachable!(),
                });
                self.stack_pop();
                out
            },
            BinaryOp::Equal => {
                let label_true = self.codegen.create_label();
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // if out is 0, jump to label_true
                self.codegen.push_jump(label_true, JumpZero::new(out, 0).into());
                // set out to false (0)
                self.codegen.push(LiteralCreate::new(0.into(), out).into());
                // jump to label_next
                self.codegen.push_jump(label_next, Jump::new(0).into());
                // set out to true (1)
                self.codegen.label_here(label_true);
                self.codegen.push(LiteralCreate::new(1.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                out
            },
            BinaryOp::NotEqual => {
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // if out is 0, jump to label_next
                self.codegen.push_jump(label_next, JumpZero::new(out, 0).into());
                // set out to true (1)
                self.codegen.push(LiteralCreate::new(1.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                out
            },
            BinaryOp::Greater => {
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                let flip = self.stack_push();
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // set flip to -out
                self.codegen.push(Neg::new(out, flip).into());
                // if flip is negative, jump to label_next (since out is already true (1))
                self.codegen.push_jump(label_next, JumpNeg::new(flip, 0).into());
                // set out to false (0)
                self.codegen.push(LiteralCreate::new(0.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                self.stack_pop();
                out
            },
            BinaryOp::GreaterOrEqual => {
                let label_false = self.codegen.create_label();
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // if out is negative, jump to label_false
                self.codegen.push_jump(label_false, JumpNeg::new(out, 0).into());
                // set out to true (1)
                self.codegen.push(LiteralCreate::new(1.into(), out).into());
                // jump to label_next
                self.codegen.push_jump(label_next, Jump::new(0).into());
                // set out to false (0)
                self.codegen.label_here(label_false);
                self.codegen.push(LiteralCreate::new(0.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                out
            },
            BinaryOp::Less => {
                let label_true = self.codegen.create_label();
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // if out is negative, jump to label_true
                self.codegen.push_jump(label_true, JumpNeg::new(out, 0).into());
                // set out to false (0)
                self.codegen.push(LiteralCreate::new(0.into(), out).into());
                // jump to label_next
                self.codegen.push_jump(label_next, Jump::new(0).into());
                // set out to true (1)
                self.codegen.label_here(label_true);
                self.codegen.push(LiteralCreate::new(1.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                out
            },
            BinaryOp::LessOrEqual => {
                let label_false = self.codegen.create_label();
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let rhs = self.compile_expr(rhs);
                // set out to -1 if a < b, 0 if a == b, 1 if a > b
                self.codegen.push(Cmp::new(lhs, rhs, out).into());
                // set out to -out
                self.codegen.push(Neg::new(out, out).into());
                // if out is negative, jump to label_false
                self.codegen.push_jump(label_false, JumpNeg::new(out, 0).into());
                // set out to true (1)
                self.codegen.push(LiteralCreate::new(1.into(), out).into());
                // jump to label_next
                self.codegen.push_jump(label_next, Jump::new(0).into());
                // set out to false (0)
                self.codegen.label_here(label_false);
                self.codegen.push(LiteralCreate::new(0.into(), out).into());
                self.codegen.label_here(label_next);
                self.stack_pop();
                out
            },
            BinaryOp::LogicAnd => {
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                self.stack_pop();
                // copy lhs to out
                self.codegen.push(LocalCopy::new(lhs, out).into());
                // if out is zero, jump to label_next (since out is already false (0))
                self.codegen.push_jump(label_next, JumpZero::new(lhs, 0).into());
                // rhs codegen
                let rhs = self.compile_expr(rhs);
                // copy rhs to out
                self.codegen.push(LocalCopy::new(rhs, out).into());
                self.codegen.label_here(label_next);
                out
            },
            BinaryOp::LogicOr => {
                let label_next = self.codegen.create_label();
                let lhs = self.compile_expr(lhs);
                let out = self.stack_push();
                let flip = self.stack_push();
                self.stack_pop();
                self.stack_pop();
                // copy lhs to out
                self.codegen.push(LocalCopy::new(lhs, out).into());
                // set flip to -out
                self.codegen.push(Neg::new(out, flip).into());
                // if flip is neg, jump to label_next (since out is already true (1))
                self.codegen.push_jump(label_next, JumpNeg::new(flip, 0).into());
                // rhs codegen
                let rhs = self.compile_expr(rhs);
                // copy rhs to out
                self.codegen.push(LocalCopy::new(rhs, out).into());
                self.codegen.label_here(label_next);
                out
            },
        }
    }
}
