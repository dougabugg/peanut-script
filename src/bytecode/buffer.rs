use std::convert::TryInto;

use crate::datamodel::Buffer;

use super::{CallFrame, DataIO, OpAction, OpError, Operation};

new_unary_op!(BufferCreate);
impl Operation for BufferCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let len = *TryInto::<&i64>::try_into(m.load(self.val as usize)?)?;
        let val = Buffer::new();
        val.resize(len as usize);
        m.store(self.out as usize, val.into())?;
        Ok(OpAction::None)
    }
}

pub struct BufferGetSlice {
    buffer: u8,
    a: u8,
    b: u8,
    out: u8,
}

impl DataIO for BufferGetSlice {
    type Target = (u8, u8, u8, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(BufferGetSlice {
            buffer: t.0,
            a: t.1,
            b: t.2,
            out: t.3,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.buffer, self.a, self.b, self.out)
    }
}

impl Operation for BufferGetSlice {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let buffer: &Buffer = m.load(self.buffer as usize)?.try_into()?;
        let a = *TryInto::<&i64>::try_into(m.load(self.a as usize)?)? as usize;
        let b = *TryInto::<&i64>::try_into(m.load(self.b as usize)?)? as usize;
        let slice = buffer.get_slice(a, b).ok_or(OpError::IndexRead(b))?;
        m.store(self.out as usize, slice.into())?;
        Ok(OpAction::None)
    }
}

pub struct BufferSetSlice {
    buffer: u8,
    src: u8,
    src_offset: u8,
    offset: u8,
    len: u8,
}

impl DataIO for BufferSetSlice {
    type Target = (u8, u8, u8, u8, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(BufferSetSlice {
            buffer: t.0,
            src: t.1,
            src_offset: t.2,
            offset: t.3,
            len: t.4,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (
            self.buffer,
            self.src,
            self.src_offset,
            self.offset,
            self.len,
        )
    }
}

impl Operation for BufferSetSlice {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let buffer: &Buffer = m.load(self.buffer as usize)?.try_into()?;
        let src: &Buffer = m.load(self.src as usize)?.try_into()?;
        let src_offset = *TryInto::<&i64>::try_into(m.load(self.src_offset as usize)?)? as usize;
        let offset = *TryInto::<&i64>::try_into(m.load(self.offset as usize)?)? as usize;
        let len = *TryInto::<&i64>::try_into(m.load(self.len as usize)?)? as usize;
        buffer
            .set_slice(src, src_offset, offset, len)
            .ok_or(OpError::IndexWrite(len))?;
        Ok(OpAction::None)
    }
}
