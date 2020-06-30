use std::convert::TryInto;

use crate::datamodel::{Identity, List, Value};

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

fn seq_to_vec(seq: &Value) -> Result<Vec<Value>, OpError> {
    Ok(match seq {
        Value::Tuple(t) => t.as_slice().to_vec(),
        Value::Record(t) => t.iter().collect(),
        Value::Table(t) => t.to_vec(),
        Value::List(t) => t.as_slice().to_vec(),
        Value::Buffer(t) => t.as_slice().iter().map(|b| (*b as i64).into()).collect(),
        _ => return Err(OpError::BadType(seq.get_inner_type_name())),
    })
}

new_unary_op!(SeqToList);
impl Operation for SeqToList {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.val as usize)?;
        let list = List::new(seq_to_vec(seq)?);
        m.store(self.out as usize, list.into())?;
        Ok(OpAction::None)
    }
}

new_bin_op!(SeqAppend);
impl Operation for SeqAppend {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let seq = m.load(self.lhs as usize)?;
        let src = m.load(self.rhs as usize)?;
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
            _ => return Err(OpError::BadType(seq.get_inner_type_name())),
        }
        Ok(OpAction::None)
    }
}
