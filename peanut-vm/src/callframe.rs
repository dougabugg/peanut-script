use crate::bytecode::{OpAction, OpError, Operation};
use crate::datamodel::{Function, Value};

use super::CallStack;

pub struct CallFrame {
    pub parent: Option<Box<CallFrame>>,
    pub function: Function,
    pub cursor: usize,
    pub stack: CallStack,
    pub output: u8,
}

impl CallFrame {
    pub fn new(function: Function) -> CallFrame {
        let stack = CallStack::new(function.stack_size);
        CallFrame {
            parent: None,
            function,
            cursor: 0,
            stack,
            output: 0,
        }
    }

    pub fn store(&mut self, index: u8, val: Value) -> Result<(), OpError> {
        self.stack.store(index, val)
    }

    pub fn jump(&mut self, index: usize) {
        self.cursor = index;
    }

    pub fn exec(&mut self) -> Result<OpAction, OpError> {
        let op = match self.function.ops.get(self.cursor) {
            Some(op) => op.clone(),
            None => return Ok(OpAction::Return(Value::None)),
        };
        self.cursor += 1;
        op.exec(&mut self.stack)
    }
}
