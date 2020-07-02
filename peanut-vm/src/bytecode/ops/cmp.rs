use std::cmp::Ordering;
use std::convert::TryInto;

use crate::datamodel::{
    Buffer, Function, Identity, List, NativeFn, Table, Unknown, Value, ValueTryIntoError,
};

use super::{CallStack, DataIO, OpAction, OpError, Operation};

new_bin_op!(Cmp);
impl Operation for Cmp {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let lhs: &Value = m.load(self.lhs)?;
        let rhs: &Value = m.load(self.rhs)?;
        let result = match lhs {
            Value::None => match rhs {
                Value::None => Ordering::Equal.into(),
                _ => Ordering::Less.into(),
            },
            Value::Integer(lhs) => lhs.cmp(rhs.try_into()?).into(),
            Value::Real(lhs) => lhs.partial_cmp(rhs.try_into()?).into(),
            Value::Tuple(lhs) => cmp_tuple(lhs.identity(), rhs)?.into(),
            Value::TupleWeak(lhs) => cmp_tuple(lhs.identity(), rhs)?.into(),
            Value::Table(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Table>::try_into(rhs)?.identity())
                .into(),
            Value::List(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&List>::try_into(rhs)?.identity())
                .into(),
            Value::Buffer(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Buffer>::try_into(rhs)?.identity())
                .into(),
            Value::Function(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Function>::try_into(rhs)?.identity())
                .into(),
            Value::NativeFn(lhs) => lhs.cmp(TryInto::<&NativeFn>::try_into(rhs)?).into(),
            Value::Unknown(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Unknown>::try_into(rhs)?.identity())
                .into(),
        };
        m.store(self.out, result)?;
        Ok(OpAction::None)
    }
}

fn cmp_tuple(lhs: usize, rhs: &Value) -> Result<Ordering, ValueTryIntoError> {
    Ok(match rhs {
        Value::Tuple(rhs) => lhs.cmp(&rhs.identity()),
        Value::TupleWeak(rhs) => lhs.cmp(&rhs.identity()),
        _ => {
            let e = ValueTryIntoError {
                found: rhs.get_inner_type_name(),
                expected: "Tuple",
            };
            return Err(e);
        }
    })
}
