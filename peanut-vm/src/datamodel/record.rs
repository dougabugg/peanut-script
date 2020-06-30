use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::{Identity, Value};

#[derive(Clone)]
pub struct Record {
    items: Rc<[RefCell<Value>]>,
}

impl Record {
    pub fn new(items: Vec<RefCell<Value>>) -> Record {
        Record {
            items: Rc::from(items),
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = Value>) -> Record {
        let items = iter.map(|v| RefCell::new(v)).collect();
        Record::new(items)
    }

    pub fn empty(n: usize) -> Record {
        let mut v = Vec::with_capacity(n);
        v.resize_with(n, || RefCell::new(Value::None));
        Record::new(v)
    }

    pub fn downgrade(&self) -> WeakRecord {
        WeakRecord {
            weakref: Rc::downgrade(&self.items),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        Some(self.items.get(index)?.borrow().clone())
    }

    pub fn set(&self, index: usize, value: Value) -> Option<Value> {
        Some(self.items.get(index)?.replace(value))
    }
}

impl Identity for Record {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items).cast::<RefCell<Value>>() as usize
    }
}

/*
TODO IDEA for handling cycles. impl Drop for Record to check if the items Rc has no other strong refs.
if no more strong refs, then inform allocator to remove the records Weak ref from it's list, so it can be
de-allocated. When a record is created, it is added to the
*/

#[derive(Clone)]
pub struct WeakRecord {
    weakref: Weak<[RefCell<Value>]>,
}

impl WeakRecord {
    pub fn upgrade(&self) -> Option<Record> {
        Some(Record {
            items: self.weakref.upgrade()?,
        })
    }
}

// Weak::as_ptr not implemented for [T] since it is unsized
// see https://users.rust-lang.org/t/why-is-rc-as-ptr-impl-for-sized-but-not-weak-as-ptr/45059
// impl Identity for WeakRecord {
//     fn identity(&self) -> usize {
//         Weak::as_ptr(&self.weakref) as usize
//     }
// }

impl Identity for WeakRecord {
    fn identity(&self) -> usize {
        match self.weakref.upgrade() {
            Some(strong) => Rc::as_ptr(&strong).cast::<RefCell<Value>>() as usize,
            None => 0,
        }
    }
}