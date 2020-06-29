use std::cmp::Ordering;
use std::convert::TryInto;

use crate::datamodel::{
    Buffer, Function, Identity, List, Table, Tuple, Unknown, Value, ValueTryIntoError,
};

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

new_bin_op!(Cmp);
impl Operation for Cmp {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let lhs: &Value = m.local.get(self.lhs as usize).ok_or(OpError::StackRead)?;
        let rhs: &Value = m.local.get(self.rhs as usize).ok_or(OpError::StackRead)?;
        let result = match lhs {
            Value::None => match rhs {
                Value::None => Ordering::Equal.into(),
                _ => {
                    let e = ValueTryIntoError {
                        found: rhs.get_inner_type_name(),
                        expected: Value::None.get_inner_type_name(),
                    };
                    return Err(OpError::IntoType(e));
                }
            },
            Value::Bool(lhs) => lhs.cmp(rhs.try_into()?).into(),
            Value::Integer(lhs) => lhs.cmp(rhs.try_into()?).into(),
            Value::Real(lhs) => lhs.partial_cmp(rhs.try_into()?).into(),
            Value::Tuple(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Tuple>::try_into(rhs)?.identity())
                .into(),
            Value::Record(lhs) => cmp_record(lhs.identity(), rhs)?.into(),
            Value::WeakRecord(lhs) => cmp_record(lhs.identity(), rhs)?.into(),
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
            Value::Unknown(lhs) => lhs
                .identity()
                .cmp(&TryInto::<&Unknown>::try_into(rhs)?.identity())
                .into(),
        };
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = result;
        Ok(OpAction::None)
    }
}

fn cmp_record(lhs: usize, rhs: &Value) -> Result<Ordering, ValueTryIntoError> {
    Ok(match rhs {
        Value::Record(rhs) => lhs.cmp(&rhs.identity()),
        Value::WeakRecord(rhs) => lhs.cmp(&rhs.identity()),
        _ => {
            let e = ValueTryIntoError {
                found: rhs.get_inner_type_name(),
                expected: "Record",
            };
            return Err(e);
        }
    })
}
