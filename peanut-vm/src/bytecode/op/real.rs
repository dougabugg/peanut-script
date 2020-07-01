use std::convert::TryInto;

use crate::datamodel::{Real, Value};

use super::{CallStack, DataIO, OpAction, OpError, Operation};

macro_rules! impl_real_op {
    ($name:ident, $e:expr) => {
        new_unary_op!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                let val: &Value = m.load(self.val)?;
                let val = *TryInto::<&Real>::try_into(val)?;
                m.store(self.out, $e(val).into())?;
                Ok(OpAction::None)
            }
        }
    };
}

impl_real_op!(Floor, |val: f64| val.floor());
impl_real_op!(Ceil, |val: f64| val.ceil());
impl_real_op!(Trunc, |val: f64| val.trunc());
impl_real_op!(Round, |val: f64| val.round());
