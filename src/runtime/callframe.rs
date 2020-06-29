use crate::bytecode::OpError;
use crate::datamodel::Value;

pub struct CallFrame<'a> {
    pub parent: Option<&'a CallFrame<'a>>,
    pub bytecode: &'a [u8],
    pub cursor: usize,
    pub local: Vec<Value>,
    pub output: u8,
}

impl<'a> CallFrame<'a> {
    pub fn load(&self, index: usize) -> Result<&Value, OpError> {
        self.local.get(index).ok_or(OpError::StackRead)
    }

    pub fn store(&mut self, index: usize, val: Value) -> Result<(), OpError> {
        let out = self.local.get_mut(index).ok_or(OpError::StackWrite)?;
        *out = val;
        Ok(())
    }
}
