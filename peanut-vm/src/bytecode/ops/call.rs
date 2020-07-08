use crate::datamodel::Value;

use super::{CallStack, OpAction, OpError, Operation};

new_op! {
    pub struct Call {
        args: u8,
    }
}

impl Operation for Call {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let target = m.pop()?;
        let mut args = Vec::new();
        for _ in 0..self.args {
            let val = m.pop()?;
            args.push(val);
        }
        match target {
            Value::Function(t) => Ok(OpAction::Call(t, args)),
            Value::NativeFn(t) => Ok(OpAction::CallNative(t, args)),
            _ => Err(OpError::BadType(target.get_type())),
        }
    }
}

new_op_empty!(Return);
impl Operation for Return {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let output = m.pop()?;
        Ok(OpAction::Return(output))
    }
}
