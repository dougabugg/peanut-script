use crate::datamodel::Value;

use super::{BytesIO, BytesReadError, CallFrame, DataIO, OpAction, OpError, Operation};

new_unary_op!(LocalCopy);
impl Operation for LocalCopy {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let val = m.load(self.val as usize)?.clone();
        m.store(self.out as usize, val)?;
        Ok(OpAction::None)
    }
}

#[derive(Clone, Copy)]
pub enum LiteralValue {
    None,
    Bool(bool),
    Integer(i64),
    Real(f64),
}

impl BytesIO for LiteralValue {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b2, n) = <u8 as BytesIO>::read(b)?;
        match n {
            0 => Ok((b2, LiteralValue::None)),
            1 => {
                let (b, bl) = <u8 as BytesIO>::read(b2)?;
                Ok((b, LiteralValue::Bool(bl != 0)))
            }
            2 => {
                let (b, int) = <i64 as BytesIO>::read(b2)?;
                Ok((b, LiteralValue::Integer(int)))
            }
            3 => {
                let (b, real) = <f64 as BytesIO>::read(b2)?;
                Ok((b, LiteralValue::Real(real)))
            }
            _ => Err(BytesReadError::InvalidValue(b)),
        }
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        match t {
            LiteralValue::None => Some(<u8 as BytesIO>::write(&0, b)?),
            LiteralValue::Bool(bl) => Some(<(u8, u8) as BytesIO>::write(&(1, *bl as u8), b)?),
            LiteralValue::Integer(int) => Some(<(u8, i64) as BytesIO>::write(&(2, *int), b)?),
            LiteralValue::Real(real) => Some(<(u8, f64) as BytesIO>::write(&(3, *real), b)?),
        }
    }
}

pub struct LiteralCreate {
    val: LiteralValue,
    out: u8,
}

impl DataIO for LiteralCreate {
    type Target = (LiteralValue, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(LiteralCreate { val: t.0, out: t.1 })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.val, self.out)
    }
}

impl Operation for LiteralCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let out: &mut Value = m
            .local
            .get_mut(self.out as usize)
            .ok_or(OpError::StackWrite)?;
        *out = match self.val {
            LiteralValue::None => Value::None,
            LiteralValue::Bool(bl) => bl.into(),
            LiteralValue::Integer(int) => int.into(),
            LiteralValue::Real(real) => real.into(),
        };
        Ok(OpAction::None)
    }
}
