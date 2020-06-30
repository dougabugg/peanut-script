use std::convert::TryInto;

use crate::datamodel::{Integer, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

macro_rules! impl_int_op {
    ($name:ident, $e:expr) => {
        new_bin_op!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
                let lhs: &Value = m.load(self.lhs as usize)?;
                let rhs: &Value = m.load(self.rhs as usize)?;
                let lhs = *TryInto::<&Integer>::try_into(lhs)?;
                let rhs = *TryInto::<&Integer>::try_into(rhs)?;
                let result = $e(lhs, rhs).into();
                m.store(self.out as usize, result)?;
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
    fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val as usize)?;
        let val = *TryInto::<&Integer>::try_into(val)?;
        m.store(self.out as usize, (!val).into())?;
        Ok(OpAction::None)
    }
}
