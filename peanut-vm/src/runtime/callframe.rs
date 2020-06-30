use crate::bytecode::OpError;
use crate::datamodel::{Function, Value};

pub struct CallFrame {
    pub parent: Option<Box<CallFrame>>,
    pub function: Function,
    pub cursor: usize,
    pub stack: Vec<Value>,
}

impl CallFrame {
    pub fn new(function: Function) -> CallFrame {
        let mut stack = Vec::new();
        stack.resize_with(function.stack_size as usize, || Value::None);
        CallFrame {
            parent: None,
            function,
            cursor: 0,
            stack
        }
    }

    pub fn load(&self, index: usize) -> Result<&Value, OpError> {
        self.stack.get(index).ok_or(OpError::StackRead(index))
    }

    pub fn store(&mut self, index: usize, val: Value) -> Result<(), OpError> {
        let out = self
            .stack
            .get_mut(index)
            .ok_or(OpError::StackWrite(index))?;
        *out = val;
        Ok(())
    }
}
