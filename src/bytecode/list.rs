use std::convert::TryInto;

use crate::datamodel::{List, Value};

use super::{CallFrame, DataIO, OpAction, OpError, Operation, StackArgs};

pub struct ListCreate {
    items: Vec<u8>,
    out: u8,
}

impl DataIO for ListCreate {
    type Target = (StackArgs, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(ListCreate {
            items: t.0.unwrap(),
            out: t.1,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (StackArgs::new(self.items.clone()), self.out)
    }
}

impl Operation for ListCreate {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for i in &self.items {
            let item = m.load(*i as usize)?;
            acc.push(item.clone());
        }
        m.store(self.out as usize, List::new(acc).into())?;
        Ok(OpAction::None)
    }
}

new_unary_op!(ListPush);
impl Operation for ListPush {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let list: &List = m.load(self.val as usize)?.try_into()?;
        let val: &Value = m.load(self.out as usize)?;
        list.push(val.clone());
        Ok(OpAction::None)
    }
}

new_unary_op!(ListPop);
impl Operation for ListPop {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let list: &List = m.load(self.val as usize)?.try_into()?;
        let val = match list.pop() {
            Some(val) => val,
            None => return Err(OpError::IndexWrite),
        };
        m.store(self.out as usize, val)?;
        Ok(OpAction::None)
    }
}

pub struct ListGetSlice {
    list: u8,
    a: u8,
    b: u8,
    out: u8,
}

impl DataIO for ListGetSlice {
    type Target = (u8, u8, u8, u8);
    fn from_bytes(t: Self::Target) -> Option<Self> {
        Some(ListGetSlice {
            list: t.0,
            a: t.1,
            b: t.2,
            out: t.3,
        })
    }
    fn into_bytes(&self) -> Self::Target {
        (self.list, self.a, self.b, self.out)
    }
}

impl Operation for ListGetSlice {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
        let list: &List = m.load(self.list as usize)?.try_into()?;
        let a = *TryInto::<&i64>::try_into(m.load(self.a as usize)?)?;
        let b = *TryInto::<&i64>::try_into(m.load(self.b as usize)?)?;
        let a: usize = a.try_into().or(Err(OpError::IndexRead))?;
        let b: usize = b.try_into().or(Err(OpError::IndexRead))?;
        let slice = list.get_slice(a, b).ok_or(OpError::IndexRead)?;
        m.store(self.out as usize, slice.into())?;
        Ok(OpAction::None)
    }
}

// pub struct ListSetSlice {
//     list: u8,
//     src: u8,
//     src_offset: u8,
//     offset: u8,
//     len: u8,
// }

// impl DataIO for ListSetSlice {
//     type Target = (u8, u8, u8, u8, u8);
//     fn from_bytes(t: Self::Target) -> Option<Self> {
//         Some(ListSetSlice {
//             list: t.0,
//             src: t.1,
//             src_offset: t.2,
//             offset: t.3,
//             len: t.4,
//         })
//     }
//     fn into_bytes(&self) -> Self::Target {
//         (self.list, self.src, self.src_offset, self.offset, self.len)
//     }
// }

// impl Operation for ListSetSlice {
//     fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
//         let list: &List = m.load(self.list as usize)?.try_into()?;
//         let src: &List = m.load(self.src as usize)?.try_into()?;
//         let src_offset: i64 = *m.load(self.src_offset as usize)?.try_into()?;
//         let offset: i64 = *m.load(self.offset as usize)?.try_into()?;
//         let len: i64 = *m.load(self.len as usize)?.try_into()?;
//         let src_offset: usize = src_offset.try_into().or(Err(OpError::IndexRead))?;
//         let offset: usize = offset.try_into().or(Err(OpError::IndexRead))?;
//         let len: usize = len.try_into().or(Err(OpError::IndexRead))?;
//         list.set_slice(src, src_offset, offset, len).ok_or(OpError::IndexWrite)?;
//     }
// }
