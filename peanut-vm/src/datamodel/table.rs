use std::cell::RefCell;
use std::rc::Rc;

use super::{Identity, Value};

#[derive(Clone)]
pub struct Table {
    items: Rc<[(u64, RefCell<Value>)]>,
}

impl Table {
    pub fn new(mut items: Vec<(u64, RefCell<Value>)>) -> Table {
        items.sort_unstable_by_key(|(k, _v)| *k);
        Table {
            items: Rc::from(items),
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = (u64, Value)>) -> Table {
        let items = iter.map(|(k, v)| (k, RefCell::new(v))).collect();
        Table::new(items)
    }

    fn get_cell(&self, key: u64) -> Option<&RefCell<Value>> {
        let index = self.items.binary_search_by_key(&key, |(k, _v)| *k).ok()?;
        Some(&self.items[index].1)
    }

    pub fn get(&self, key: u64) -> Option<Value> {
        let cell = self.get_cell(key)?;
        Some(cell.borrow().clone())
    }

    pub fn set(&self, key: u64, value: Value) -> Option<Value> {
        Some(self.get_cell(key)?.replace(value))
    }
}

impl Identity for Table {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items).cast::<(u64, RefCell<Value>)>() as usize
    }
}
