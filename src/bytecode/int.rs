use std::convert::TryInto;

use crate::datamodel::{Integer, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

macro_rules! impl_int_op {
    ($name:ident, $e:expr) => {
        new_bin_op!($name);
        impl Operation for $name {
            fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
                let lhs: &Value = m.local.get(self.lhs as usize).ok_or(OpError::StackRead)?;
                let rhs: &Value = m.local.get(self.rhs as usize).ok_or(OpError::StackRead)?;
                let lhs = *TryInto::<&Integer>::try_into(lhs)?;
                let rhs = *TryInto::<&Integer>::try_into(rhs)?;
                let result = $e(lhs, rhs).into();
                let out: &mut Value = m.local.get_mut(self.out as usize).ok_or(OpError::StackWrite)?;
                *out = result;
                Ok(OpAction::None)
            }
        }
    };
}

impl_int_op!(Shl, |lhs, rhs| lhs << rhs);
impl_int_op!(Shr, |lhs, rhs| lhs >> rhs);
impl_int_op!(And, |lhs, rhs| lhs & rhs);
impl_int_op!(Or, |lhs, rhs| lhs | rhs);
impl_int_op!(Xor, |lhs, rhs| lhs ^ rhs);

new_unary_op!(Not);
impl Operation for Not {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val: &Value = m.local.get(self.val as usize).ok_or(OpError::StackRead)?;
        let val = *TryInto::<&Integer>::try_into(val)?;
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = (!val).into();
        Ok(OpAction::None)
    }
}
