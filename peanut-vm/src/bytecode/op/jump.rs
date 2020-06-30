use crate::datamodel::Value;

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

pub struct Jump {
    dest: u32,
}

impl DataIO for Jump {
    type Target = u32;
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(Jump { dest: t })
    }
    fn into_bytes(&self) -> Self::Target {
        self.dest
    }
}

impl Operation for Jump {
    fn exec(&self, _: &mut CallFrame) -> Result<OpAction, OpError> {
        Ok(OpAction::Jump(self.dest as usize))
    }
}

pub struct JumpZero {
    val: u8,
    dest: u32,
}

impl DataIO for JumpZero {
    type Target = (u8, u32);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(JumpZero {
            val: t.0,
            dest: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.val, self.dest)
    }
}

impl Operation for JumpZero {
    fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val as usize)?;
        let is_zero = match val {
            Value::None => true,
            Value::Integer(i) => *i == 0,
            Value::Real(r) => *r == 0.0,
            _ => false,
        };
        if is_zero {
            Ok(OpAction::Jump(self.dest as usize))
        } else {
            Ok(OpAction::None)
        }
    }
}

pub struct JumpNeg {
    val: u8,
    dest: u32,
}

impl DataIO for JumpNeg {
    type Target = (u8, u32);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(JumpNeg {
            val: t.0,
            dest: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.val, self.dest)
    }
}

impl Operation for JumpNeg {
    fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val as usize)?;
        let is_zero = match val {
            Value::Integer(i) => *i < 0,
            Value::Real(r) => *r < 0.0,
            _ => false,
        };
        if is_zero {
            Ok(OpAction::Jump(self.dest as usize))
        } else {
            Ok(OpAction::None)
        }
    }
}
