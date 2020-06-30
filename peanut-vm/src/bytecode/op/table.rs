use std::cell::RefCell;
use std::convert::TryInto;

use crate::datamodel::{Integer, List, Table, Tuple, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

new_unary_op!(TableCreate);
impl Operation for TableCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val: &Value = m.load(self.val as usize)?;
        let list: &List = val.try_into()?;
        let mut table = Vec::new();
        for val in list.as_slice().iter() {
            let tuple: &Tuple = val.try_into()?;
            let k: Integer = tuple.get(0).ok_or(OpError::IndexRead(0))?.try_into()?;
            let v: Value = tuple.get(1).ok_or(OpError::IndexRead(1))?;
            table.push((k as u64, RefCell::new(v)));
        }
        let table = Table::new(table);
        m.store(self.out as usize, table.into())?;
        Ok(OpAction::None)
    }
}

new_bin_op!(TableGet);
impl Operation for TableGet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let table: &Table = m.load(self.lhs as usize)?.try_into()?;
        let key: &Integer = m.load(self.rhs as usize)?.try_into()?;
        let val = table.get(*key as u64).unwrap_or(Value::None);
        m.store(self.out as usize, val)?;
        Ok(OpAction::None)
    }
}

new_bin_op!(TableSet);
impl Operation for TableSet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let table: &Table = m.load(self.lhs as usize)?.try_into()?;
        let key: &Integer = m.load(self.rhs as usize)?.try_into()?;
        let val = m.load(self.out as usize)?;
        // just pass 0 instead of `*key as usize` because on 32-bit platforms it
        // would truncate the value
        table
            .set(*key as u64, val.clone())
            .ok_or(OpError::IndexWrite(0))?;
        Ok(OpAction::None)
    }
}
