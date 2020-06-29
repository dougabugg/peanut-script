use std::ops::Deref;

use super::{BytesIO, BytesReadError};

pub struct StackArgs {
    items: Vec<u8>,
}

impl StackArgs {
    pub fn new(items: Vec<u8>) -> StackArgs {
        StackArgs { items }
    }

    pub fn unwrap(self) -> Vec<u8> {
        self.items
    }
}

impl Deref for StackArgs {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl BytesIO for StackArgs {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, n) = <u8 as BytesIO>::read(b)?;
        let b1 = b.get(n as usize..).ok_or(BytesReadError::EndOfFile)?;
        let b2 = unsafe { b.get_unchecked(..n as usize) };
        Ok((b1, StackArgs { items: b2.to_vec() }))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let n = t.items.len();
        if n > u8::MAX as usize {
            panic!("error: operations cannot have more than 255 stack arguments");
        }
        let n = n as u8;
        let b = <u8 as BytesIO>::write(&n, b)?;
        b.get_mut(..n as usize)?.copy_from_slice(&t.items);
        Some(unsafe { b.get_unchecked_mut(n as usize..) })
    }
}
