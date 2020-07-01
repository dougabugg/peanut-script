use std::convert::TryInto;

use crate::datamodel::Buffer;

use super::{CallStack, DataIO, OpAction, OpError, Operation};

new_unary_op!(BufferCreate);
impl Operation for BufferCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let len = *TryInto::<&i64>::try_into(m.load(self.val)?)?;
        let val = Buffer::empty();
        val.resize(len as usize);
        m.store(self.out, val.into())?;
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct BufferGetSlice {
        buffer: u8,
        a: u8,
        b: u8,
        out: u8,
    }
}

impl Operation for BufferGetSlice {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let buffer: &Buffer = m.load(self.buffer)?.try_into()?;
        let a = *TryInto::<&i64>::try_into(m.load(self.a)?)? as usize;
        let b = *TryInto::<&i64>::try_into(m.load(self.b)?)? as usize;
        let slice = buffer.get_slice(a, b).ok_or(OpError::IndexRead(b))?;
        m.store(self.out, slice.into())?;
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct BufferSetSlice {
        buffer: u8,
        src: u8,
        src_offset: u8,
        offset: u8,
        len: u8,
    }
}

impl Operation for BufferSetSlice {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let buffer: &Buffer = m.load(self.buffer)?.try_into()?;
        let src: &Buffer = m.load(self.src)?.try_into()?;
        let src_offset = *TryInto::<&i64>::try_into(m.load(self.src_offset)?)? as usize;
        let offset = *TryInto::<&i64>::try_into(m.load(self.offset)?)? as usize;
        let len = *TryInto::<&i64>::try_into(m.load(self.len)?)? as usize;
        buffer
            .set_slice(src, src_offset, offset, len)
            .ok_or(OpError::IndexWrite(len))?;
        Ok(OpAction::None)
    }
}
