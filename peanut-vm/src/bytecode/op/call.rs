use crate::datamodel::Value;

use super::{CallFrame, DataIO, OpAction, OpError, Operation, StackArgs};

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
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let target = m.load(self.target as usize)?;
        let mut args = Vec::new();
        for i in &self.args {
            let val = m.load(*i as usize)?.clone();
            args.push(val);
        }
        match target {
            Value::Function(t) => Ok(OpAction::Call(t.clone(), args, self.output)),
            Value::NativeFn(t) => Ok(OpAction::CallNative(*t, args, self.output)),
            _ => Err(OpError::BadType(target.get_inner_type_name())),
        }
    }
}

pub struct Return {
    output: u8,
}

impl DataIO for Return {
    type Target = u8;
    fn from_bytes(output: Self::Target) -> Option<Self> {
        Some(Return { output })
    }
    fn into_bytes(&self) -> Self::Target {
        self.output
    }
}

impl Operation for Return {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let output = m.load(self.output as usize)?.clone();
        Ok(OpAction::Return(output))
    }
}
