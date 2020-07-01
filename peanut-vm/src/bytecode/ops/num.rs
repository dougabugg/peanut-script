use std::convert::TryInto;

use crate::datamodel::{Integer, Real, Value, ValueTryIntoError};

use super::{CallStack, DataIO, OpAction, OpError, Operation};

pub const NUM_TYPE_NAME: &'static str = "Integer or Real";

macro_rules! impl_math_op {
    ($name:ident, $e:expr) => {
        new_bin_op!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                let lhs: &Value = m.load(self.lhs)?;
                let rhs: &Value = m.load(self.rhs)?;
                let result = match lhs {
                    Value::Integer(lhs) => {
                        let rhs = *TryInto::<&Integer>::try_into(rhs)?;
                        $e(lhs, rhs).into()
                    }
                    Value::Real(lhs) => {
                        let rhs = *TryInto::<&Real>::try_into(rhs)?;
                        $e(lhs, rhs).into()
                    }
                    _ => {
                        let e = ValueTryIntoError {
                            found: lhs.get_inner_type_name(),
                            expected: NUM_TYPE_NAME,
                        };
                        return Err(OpError::IntoType(e));
                    }
                };
                m.store(self.out, result)?;
                Ok(OpAction::None)
            }
        }
    };
}

impl_math_op!(Add, |lhs, rhs| lhs + rhs);
impl_math_op!(Sub, |lhs, rhs| lhs - rhs);
impl_math_op!(Mul, |lhs, rhs| lhs * rhs);
impl_math_op!(Div, |lhs, rhs| lhs / rhs);
impl_math_op!(Rem, |lhs, rhs| lhs % rhs);

new_unary_op!(Neg);
impl Operation for Neg {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let val: Value = match val {
            Value::Integer(val) => (-val).into(),
            Value::Real(val) => (-val).into(),
            _ => {
                let e = ValueTryIntoError {
                    found: val.get_inner_type_name(),
                    expected: NUM_TYPE_NAME,
                };
                return Err(OpError::IntoType(e));
            }
        };
        m.store(self.out, val)?;
        Ok(OpAction::None)
    }
}
