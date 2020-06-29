use std::rc::Rc;

use super::Identity;

use crate::bytecode::Op;

#[derive(Clone)]
pub struct Function {
    pub stack_size: u8,
    pub ops: Rc<Vec<Op>>,
}

impl Identity for Function {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.ops) as usize
    }
}
