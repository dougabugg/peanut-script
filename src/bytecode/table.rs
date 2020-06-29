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
