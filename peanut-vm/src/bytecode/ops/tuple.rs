use std::cell::RefCell;
use std::convert::TryInto;

use crate::datamodel::{List, Tuple, TupleWeak, Value};

use super::{CallStack, DataIO, OpAction, OpError, Operation, StackArgs};

pub struct TupleCreate {
    items: Vec<u8>,
    out: u8,
}

impl DataIO for TupleCreate {
    type Target = (StackArgs, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(TupleCreate {
            items: t.0.unwrap(),
            out: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (StackArgs::new(self.items.clone()), self.out)
    }
}

impl Operation for TupleCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for i in &self.items {
            let item = m.load(*i)?;
            acc.push(RefCell::new(item.clone()));
        }
        m.store(self.out, Tuple::new(acc).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(TupleFromList);
impl Operation for TupleFromList {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let list: &List = val.try_into()?;
        let tuple = Tuple::from_iter(list.as_slice().iter().map(|v| v.clone()));
        m.store(self.out, tuple.into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(TupleWeakRef);
impl Operation for TupleWeakRef {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val)?;
        let tuple: &Tuple = val.try_into()?;
        let weak = tuple.downgrade();
        m.store(self.out, weak.into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(TupleWeakUpgrade);
impl Operation for TupleWeakUpgrade {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let weak: &TupleWeak = m.load(self.val)?.try_into()?;
        let tuple = weak.upgrade();
        m.store(self.out, tuple.into())?;
        Ok(OpAction::None)
    }
}
