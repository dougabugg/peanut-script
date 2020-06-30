use std::convert::TryInto;

use crate::datamodel::{List, Tuple, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation, StackArgs};

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
    fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for i in &self.items {
            let item = m.load(*i as usize)?;
            acc.push(item.clone());
        }
        m.store(self.out as usize, Tuple::new(acc).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(TupleFromList);
impl Operation for TupleFromList {
    fn exec(&self, m: &mut CallFrame) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val as usize)?;
        let list: &List = val.try_into()?;
        let record = Tuple::new(list.as_slice().iter().map(|v| v.clone()).collect());
        m.store(self.out as usize, record.into())?;
        Ok(OpAction::None)
    }
}
