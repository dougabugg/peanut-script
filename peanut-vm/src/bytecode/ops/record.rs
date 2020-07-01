use std::cell::RefCell;
use std::convert::TryInto;

use crate::datamodel::{List, Record, Value, WeakRecord};

use super::{CallStack, DataIO, OpAction, OpError, Operation, StackArgs};

pub struct RecordCreate {
    items: Vec<u8>,
    out: u8,
}

impl DataIO for RecordCreate {
    type Target = (StackArgs, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(RecordCreate {
            items: t.0.unwrap(),
            out: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (StackArgs::new(self.items.clone()), self.out)
    }
}

impl Operation for RecordCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for i in &self.items {
            let item = m.load(*i)?;
            acc.push(RefCell::new(item.clone()));
        }
        m.store(self.out, Record::new(acc).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(RecordFromList);
impl Operation for RecordFromList {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let list: &List = val.try_into()?;
        let record = Record::from_iter(list.as_slice().iter().map(|v| v.clone()));
        m.store(self.out, record.into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(RecordWeakRef);
impl Operation for RecordWeakRef {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let record: &Record = val.try_into()?;
        let weak = record.downgrade();
        m.store(self.out, weak.into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(WeakRecordUpgrade);
impl Operation for WeakRecordUpgrade {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let weak: &WeakRecord = val.try_into()?;
        let record = weak.upgrade();
        m.store(self.out, record.into())?;
        Ok(OpAction::None)
    }
}
