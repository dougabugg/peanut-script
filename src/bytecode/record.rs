use std::convert::TryInto;
use std::cell::RefCell;

use crate::datamodel::{List, Record, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation, StackArgs};

pub struct RecordCreate {
    items: Vec<u8>,
    output: u8,
}

impl DataIO for RecordCreate {
    type Target = (StackArgs, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(RecordCreate {
            items: t.0.unwrap(),
            output: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (StackArgs::new(self.items.clone()), self.output)
    }
}

impl Operation for RecordCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for i in &self.items {
            let item = m.local.get(*i as usize).ok_or(OpError::StackRead)?;
            acc.push(RefCell::new(item.clone()));
        }
        let out: &mut Value = m.local.get_mut(self.output as usize).ok_or(OpError::StackWrite)?;
        *out = Record::new(acc);
        Ok(OpAction::None)
    }
}

new_unary_op!(RecordFromList);
impl Operation for RecordFromList {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val: &Value = m.local.get(self.val as usize).ok_or(OpError::StackRead)?;
        let list: &List = val.try_into()?;
        let record = Record::from_iter(list.as_slice().iter().map(|v| v.clone()));
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = record.into();
        Ok(OpAction::None)
    }
}

new_unary_op!(RecordWeakRef);
impl Operation for RecordWeakRef {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val: &Value = m.local.get(self.val as usize).ok_or(OpError::StackRead)?;
        let record: &Record = val.try_into()?;
        let weak = record.downgrade();
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = weak.into();
        Ok(OpAction::None)
    }
}
