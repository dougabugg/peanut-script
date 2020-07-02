use std::convert::TryInto;

use crate::datamodel::{Identity, List, Value};

use super::{CallStack, DataIO, OpAction, OpError, Operation};

new_unary_op!(SeqLen);
impl Operation for SeqLen {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.val)?;
        let len = match seq {
            Value::Tuple(t) => t.len(),
            // Value::Table(t) => t.len(),
            Value::List(t) => t.len(),
            Value::Buffer(t) => t.len(),
            _ => return Err(OpError::BadType(seq.get_type())),
        };
        m.store(self.out, (len as i64).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(SeqResize);
impl Operation for SeqResize {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.val)?;
        let len = *TryInto::<&i64>::try_into(m.load(self.out)?)? as usize;
        match seq {
            Value::List(t) => t.resize(len),
            Value::Buffer(t) => t.resize(len),
            _ => return Err(OpError::BadType(seq.get_type())),
        }
        Ok(OpAction::None)
    }
}

fn seq_get(seq: &Value, index: i64) -> Result<Value, OpError> {
    match seq {
        Value::Tuple(t) => t.get(index as usize).ok_or(OpError::IndexRead(index)),
        Value::Table(t) => Ok(t.get(index as u64).unwrap_or(Value::None)),
        Value::List(t) => t.get(index as usize).ok_or(OpError::IndexRead(index)),
        Value::Buffer(t) => t.get(index as usize).ok_or(OpError::IndexRead(index)),
        _ => return Err(OpError::BadType(seq.get_type())),
    }
}

new_bin_op!(SeqGet);
impl Operation for SeqGet {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs)?;
        let index = *TryInto::<&i64>::try_into(m.load(self.rhs)?)?;
        let val = seq_get(seq, index)?;
        m.store(self.out, val)?;
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqQuickGet);
impl Operation for SeqQuickGet {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs)?;
        let index = self.rhs as i64;
        let val = seq_get(seq, index)?;
        m.store(self.out, val.clone())?;
        Ok(OpAction::None)
    }
}

fn seq_set(seq: &Value, index: i64, val: &Value) -> Result<Value, OpError> {
    match seq {
        Value::Tuple(t) => t
            .set(index as usize, val.clone())
            .ok_or(OpError::IndexWrite(index)),
        Value::Table(t) => Ok(t.set(index as u64, val.clone()).unwrap_or(Value::None)),
        Value::List(t) => t
            .set(index as usize, val.clone())
            .ok_or(OpError::IndexWrite(index)),
        Value::Buffer(t) => t
            .set(index as usize, *TryInto::<&i64>::try_into(val)? as u8)
            .ok_or(OpError::IndexWrite(index)),
        _ => return Err(OpError::BadType(seq.get_type())),
    }
}

new_op! {
    pub struct SeqSet {
        seq: u8,
        index: u8,
        val: u8,
    }
}

impl Operation for SeqSet {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.seq)?;
        let index = *TryInto::<&i64>::try_into(m.load(self.index)?)?;
        let val = m.load(self.val)?;
        seq_set(seq, index, val)?;
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct SeqQuickSet {
        seq: u8,
        index: u8,
        val: u8,
    }
}

impl Operation for SeqQuickSet {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.seq)?;
        let index = self.index as i64;
        let val = m.load(self.val)?;
        seq_set(seq, index, val)?;
        Ok(OpAction::None)
    }
}

fn seq_to_vec(seq: &Value) -> Result<Vec<Value>, OpError> {
    Ok(match seq {
        Value::Tuple(t) => t.iter().collect(),
        Value::Table(t) => t.to_vec(),
        Value::List(t) => t.as_slice().to_vec(),
        Value::Buffer(t) => t.as_slice().iter().map(|b| (*b as i64).into()).collect(),
        _ => return Err(OpError::BadType(seq.get_type())),
    })
}

new_unary_op!(SeqToList);
impl Operation for SeqToList {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.val)?;
        let list = List::new(seq_to_vec(seq)?);
        m.store(self.out, list.into())?;
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct SeqAppend {
        seq: u8,
        src: u8,
    }
}

impl Operation for SeqAppend {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let seq = m.load(self.seq)?;
        let src = m.load(self.src)?;
        match seq {
            Value::List(list) => {
                list.append(seq_to_vec(src)?);
            }
            Value::Buffer(buffer) => match src {
                Value::Buffer(src) => {
                    if buffer.identity() == src.identity() {
                        buffer.append(&src.as_slice().to_vec());
                    } else {
                        buffer.append(&src.as_slice());
                    }
                }
                _ => {
                    let mut acc = Vec::new();
                    for val in seq_to_vec(src)?.into_iter() {
                        acc.push(TryInto::<i64>::try_into(val)? as u8)
                    }
                    buffer.append(&acc);
                }
            },
            _ => return Err(OpError::BadType(seq.get_type())),
        }
        Ok(OpAction::None)
    }
}
