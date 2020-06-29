use std::cell::RefCell;
use std::convert::TryInto;

use crate::datamodel::{Integer, List, Table, Tuple, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

new_unary_op!(TableCreate);
impl Operation for TableCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val: &Value = m.local.get(self.val as usize).ok_or(OpError::StackRead)?;
        let list: &List = val.try_into()?;
        let mut table = Vec::new();
        for val in list.as_slice().iter() {
            let tuple: &Tuple = val.try_into()?;
            let k: Integer = tuple.get(0).ok_or(OpError::IndexRead)?.try_into()?;
            let v: Value = tuple.get(1).ok_or(OpError::IndexRead)?;
            table.push((k as u64, RefCell::new(v)));
        }
        let table = Table::new(table);
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = table.into();
        Ok(OpAction::None)
    }
}
