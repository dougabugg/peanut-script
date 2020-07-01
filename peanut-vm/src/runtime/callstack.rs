use crate::bytecode::OpError;
use crate::datamodel::Value;

pub struct CallStack {
    stack: Vec<Value>,
}

impl CallStack {
    pub fn new(stack_size: u8) -> CallStack {
        let mut stack = Vec::new();
        stack.resize_with(stack_size as usize, || Value::None);
        CallStack { stack }
    }

    pub fn load(&self, index: u8) -> Result<&Value, OpError> {
        self.stack
            .get(index as usize)
            .ok_or(OpError::StackRead(index))
    }

    pub fn store(&mut self, index: u8, val: Value) -> Result<(), OpError> {
        let out = self
            .stack
            .get_mut(index as usize)
            .ok_or(OpError::StackWrite(index))?;
        *out = val;
        Ok(())
    }
}
