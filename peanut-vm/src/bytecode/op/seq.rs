use std::convert::TryInto;

use crate::datamodel::Value;

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

new_unary_op!(SeqLen);
impl Operation for SeqLen {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.val as usize)?;
        let len = match seq {
            Value::Tuple(t) => t.len(),
            Value::Record(t) => t.len(),
            // Value::Table(t) => t.len(),
            Value::List(t) => t.len(),
            Value::Buffer(t) => t.len(),
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        };
        m.store(self.out as usize, (len as i64).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(SeqResize);
impl Operation for SeqResize {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.val as usize)?;
        let len = *TryInto::<&i64>::try_into(m.load(self.out as usize)?)? as usize;
        match seq {
            Value::List(t) => t.resize(len),
            Value::Buffer(t) => t.resize(len),
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        }
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqGet);
impl Operation for SeqGet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs as usize)?;
        let index = *TryInto::<&i64>::try_into(m.load(self.rhs as usize)?)? as usize;
        let val = match seq {
            Value::Tuple(t) => t.get(index),
            Value::Record(t) => t.get(index),
            // Value::Table(t) => t.get(index),
            Value::List(t) => t.get(index),
            Value::Buffer(t) => t.get(index),
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        }
        .ok_or(OpError::IndexRead(index))?;
        m.store(self.out as usize, val.clone())?;
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqQuickGet);
impl Operation for SeqQuickGet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs as usize)?;
        let index = self.rhs as usize;
        let val = match seq {
            Value::Tuple(t) => t.get(index),
            Value::Record(t) => t.get(index),
            // Value::Table(t) => t.get(index),
            Value::List(t) => t.get(index),
            Value::Buffer(t) => t.get(index),
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        }
        .ok_or(OpError::IndexRead(index))?;
        m.store(self.out as usize, val.clone())?;
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqSet);
impl Operation for SeqSet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs as usize)?;
        let index = *TryInto::<&i64>::try_into(m.load(self.rhs as usize)?)? as usize;
        let val = m.load(self.out as usize)?;
        match seq {
            Value::List(t) => t
                .set(index, val.clone())
                .ok_or(OpError::IndexWrite(index))?,
            Value::Buffer(t) => t
                .set(index, *TryInto::<&i64>::try_into(val)? as u8)
                .ok_or(OpError::IndexWrite(index))?,
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        };
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqQuickSet);
impl Operation for SeqQuickSet {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs as usize)?;
        let index = self.rhs as usize;
        let val = m.load(self.out as usize)?;
        match seq {
            Value::List(t) => t
                .set(index, val.clone())
                .ok_or(OpError::IndexWrite(index))?,
            Value::Buffer(t) => t
                .set(index, *TryInto::<&i64>::try_into(val)? as u8)
                .ok_or(OpError::IndexWrite(index))?,
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        };
        Ok(OpAction::None)
    }
}

// TODO seq to list, and seq expand (for list and buffer)