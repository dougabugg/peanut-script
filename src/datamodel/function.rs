use std::rc::Rc;

use crate::bytecode::Op;

use super::{Identity, Record};

#[derive(Clone)]
pub struct Function {
    pub stack_size: u8,
    pub module: Record,
    pub ops: Rc<Vec<Op>>,
}

impl Function {
    pub fn new(stack_size: u8, module: Record, ops: Vec<Op>) -> Function {
        Function {
            stack_size,
            module,
            ops: Rc::new(ops),
        }
    }
}

impl Identity for Function {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.ops) as usize
    }
}
