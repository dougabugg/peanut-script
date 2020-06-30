use super::{BytesIO, BytesReadError, Op};

pub struct Function {
    pub stack_size: u8,
    pub ops: Vec<Op>,
}

impl BytesIO for Function {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, stack_size) = <u8 as BytesIO>::read(b)?;
        let (b, ops) = <Vec<Op> as BytesIO>::read(b)?;
        let f = Function { stack_size, ops };
        Ok((b, f))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let b = <u8 as BytesIO>::write(&t.stack_size, b)?;
        <Vec<Op> as BytesIO>::write(&t.ops, b)
    }
}
