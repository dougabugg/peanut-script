use crate::datamodel::Value;
// use crate::bytecode::OpError;

pub struct CallFrame<'a> {
    pub parent: Option<&'a CallFrame<'a>>,
    pub bytecode: &'a [u8],
    pub cursor: usize,
    pub local: Vec<Value>,
    pub output: u8,
}

// impl<'a> CallFrame<'a> {
//     fn load(&self, index: usize) -> Result<&Value, OpError> {
//         self.local.get(i as usize).ok_or(OpError::StackRead)
//     }

//     fn load_mut(&mut self, index: usize) -> Result<&mut Value, OpError> {
//         self.local.get_mut(i as usize).ok_or(OpError::StackRead)
//     }
// }
