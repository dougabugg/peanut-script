use std::convert::TryInto;

use crate::datamodel::Function;

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
        let target = m
            .local
            .get(self.target as usize)
            .ok_or(OpError::StackRead)?;
        let target: Function = TryInto::<&Function>::try_into(target)?.clone();
        let mut args = Vec::new();
        for i in &self.args {
            let val = m.local.get(*i as usize).ok_or(OpError::StackRead)?.clone();
            args.push(val);
        }
        m.output = self.output;
        Ok(OpAction::Call(target, args))
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
        let output = m
            .local
            .get(self.output as usize)
            .ok_or(OpError::StackRead)?
            .clone();
        Ok(OpAction::Return(output))
    }
}
