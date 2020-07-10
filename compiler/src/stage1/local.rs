use std::mem::swap;
use std::collections::BTreeSet;

struct DeferredDrop {
    location: usize,
    local: usize,
}

pub struct LocalsProcessor {
    seen: BTreeSet<usize>,
}

impl LocalsProcessor {
    pub fn process(&mut self, func: &mut Function) {
        let mut setup = Vec::new();
        for arg in func.args {
            setup.push(Statement::BindLocal(arg));
            setup.push(Statement::InitLocal(arg));
        }
        setup.append(&mut func.body);
        swap(&mut setup, &mut func.body);
        let unknown_locals = self.process_block(&mut func.body);
        if !unknown_locals.is_empty() {
            panic!("local variable access before being bound to scope");
        }
    }

    pub fn process_block(&mut self, block: &mut Vec<Statement>) -> Vec<usize> {
        let mut bindings = BTreeSet::new();
        let mut drops = Vec::new();
        for (i, statement) in block.iter_mut().enumerate().rev() {
            match statement {
                Statement::BindLocal(n) => bindings.insert(n),
                Statement::DropLocal(_) => {
                    panic!("unexpected DropLocal statement while processing local varaibles");
                }
                Statement::InitLocal(local) => {
                    let dropped = self.seen.insert(local);
                    if dropped {
                        drops.push(DeferredDrop { location: i, local });
                    }
                }
                Statement::Loop(l) => {
                    match l.condition {
                        Some(e) => {
                            for local in e.find_locals() {
                                drops.push(DeferredDrop { location: i, local });
                            }
                        }
                        None => {}
                    }
                    for local in self.process_block(&mut l.body) {
                        drops.push(DeferredDrop { location: i, local });
                    }
                },
                Statement::Other(s) => {
                    let mut locals = Vec::new();
                    match s {
                        SharedStatement::Expr(e) => locals.append(&mut e.find_locals()),
                        SharedStatement::Return(e) => locals.append(&mut e.find_locals()),
                        SharedStatement::IfElse(f) => {
                            // TODO do we need to reverse the order?
                            // so right now (if_, else_if, else_), but instead (else_, else_if, if_)?
                            // at first glance, it doesn't appear to matter, but we should take another look
                            for local in f.if_.condition.find_locals() {
                                drops.push(DeferredDrop { location: i, local });
                            }
                            for local in self.process_block(&mut f.if_.body) {
                                drops.push(DeferredDrop { location: i, local });
                            }
                            for if_ in &mut f.else_if {
                                for local in if_.condition.find_locals() {
                                    drops.push(DeferredDrop { location: i, local });
                                }
                                for local in self.process_block(&mut if_.body) {
                                    drops.push(DeferredDrop { location: i, local });
                                }
                            }
                            for local in self.process_block(&mut f.else_) {
                                drops.push(DeferredDrop { location: i, local });
                            }
                        }
                        SharedStatement::SeqAppend { seq, src } => {
                            locals.append(&mut seq.find_locals());
                            locals.append(&mut src.find_locals());
                        }
                        SharedStatement::SeqResize { seq, len } => {
                            locals.append(&mut seq.find_locals());
                            locals.append(&mut len.find_locals());
                        }
                        SharedStatement::ListPush { list, value } => {
                            locals.append(&mut list.find_locals());
                            locals.append(&mut value.find_locals());
                        }
                        SharedStatement::BufferSetSlice(b) => {
                            locals.append(&mut b.find_locals());
                        }
                    }
                    for local in locals {
                        let dropped = self.seen.insert(local);
                        if dropped {
                            drops.push(DeferredDrop { location: i, local });
                        }
                    }
                },
                _ => {}
            }
        }
        let mut parent_scope = Vec::new();
        for drop in drops.iter().rev() {
            if bindings.contains(drop.local) {
                block.insert(drop.location + 1, Statement::DropLocal(drop.local));
            } else {
                parent_scope.push(drop.local);
            }
        }
        parent_scope
    }
}

/*
ISSUES:
    there's no concept of "scope"
our model of sharing the AST between stages 0 and 1 is broken, because we need to
know the scope a variable was defined in, so we can insert DropLocal statements at
the correct location within a block. The BindLocal and DropLocal statements must
be paired and be in the same block, I think.

actually, it might be ok if we just put DropLocal statements inside of sub-blocks.
well, not quite. inside of a loop block, the DropLocal statements should be placed
outside of and following the loop block. so variable bindings do need to be block
aware.

well, we could determine if a variable was bound (via BindLocal) within a loop block,
and if it wasn't, promote the variable's DropLocal to occur after the loop.
*/