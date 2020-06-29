use std::cell::RefCell;
use std::rc::Rc;

use super::{Identity, Value};

#[derive(Clone)]
pub struct Buffer {
    items: Rc<RefCell<Vec<u8>>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            items: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn len(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn resize(&self, len: usize) {
        let mut items = self.items.borrow_mut();
        items.resize_with(len, || 0);
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        let items = self.items.borrow();
        let item = items.get(index)?;
        Some(Value::Integer(*item as i64))
    }

    pub fn set(&self, index: usize, value: u8) -> Option<Value> {
        let mut items = self.items.borrow_mut();
        let item = items.get_mut(index)?;
        let tmp = *item;
        *item = value;
        Some(Value::Integer(tmp as i64))
    }

    pub fn get_slice(&self, a: usize, b: usize) -> Option<Buffer> {
        let items = self.items.borrow();
        let v = items.get(a..b)?.to_vec();
        Some(Buffer {
            items: Rc::new(RefCell::new(v)),
        })
    }

    pub fn set_slice(
        &self,
        src: &Buffer,
        src_offset: usize,
        offset: usize,
        len: usize,
    ) -> Option<()> {
        let mut items = self.items.borrow_mut();
        if Rc::ptr_eq(&self.items, &src.items) {
            if len + src_offset > items.len() || len + offset > items.len() {
                return None;
            }
            items.copy_within(src_offset..len + src_offset, offset);
        } else {
            let dst = items.get_mut(offset..len + offset)?;
            dst.copy_from_slice(src.items.borrow().get(src_offset..len + src_offset)?);
        }
        Some(())
    }
}

impl Identity for Buffer {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items) as usize
    }
}
