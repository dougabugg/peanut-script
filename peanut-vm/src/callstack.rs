use crate::bytecode::OpError;
use crate::datamodel::Value;

pub struct CallStack {
    stack: Vec<Value>,
    locals: Vec<Value>,
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            stack: Vec::new(),
            locals: Vec::new(),
        }
    }

    pub fn load(&self, index: u8) -> Result<&Value, OpError> {
        self.locals
            .get(index as usize)
            .ok_or(OpError::LocalRead(index))
    }

    pub fn store(&mut self, index: u8, val: Value) {
        let out = match self.locals.get_mut(index as usize) {
            Some(val) => val,
            None => {
                self.locals.resize_with(index as usize + 1, || Value::None);
                unsafe { self.locals.get_unchecked_mut(index as usize) }
            }
        };
        *out = val;
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Result<Value, OpError> {
        self.stack.pop().ok_or(OpError::StackEmpty)
    }
}
