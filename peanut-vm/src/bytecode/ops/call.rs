use crate::datamodel::Value;

use super::{CallStack, DataIO, OpAction, OpError, Operation, StackArgs};

pub struct Call {
    target: u8,
    output: u8,
    args: Vec<u8>,
}

impl DataIO for Call {
    type Target = (u8, u8, StackArgs);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(Call {
            target: t.0,
            output: t.1,
            args: t.2.unwrap(),
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.target, self.output, StackArgs::new(self.args.clone()))
    }
}

impl Operation for Call {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let target = m.load(self.target)?;
        let mut args = Vec::new();
        for i in &self.args {
            let val = m.load(*i)?.clone();
            args.push(val);
        }
        match target {
            Value::Function(t) => Ok(OpAction::Call(t.clone(), args, self.output)),
            Value::NativeFn(t) => Ok(OpAction::CallNative(*t, args, self.output)),
            _ => Err(OpError::BadType(target.get_type())),
        }
    }
}

new_op! {
    pub struct Return {
        output: u8,
    }
}

impl Operation for Return {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let output = m.load(self.output)?.clone();
        Ok(OpAction::Return(output))
    }
}
