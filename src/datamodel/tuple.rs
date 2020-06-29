use std::rc::Rc;

use super::{Identity, Value};

#[derive(Clone)]
pub struct Tuple {
    items: Rc<[Value]>,
}

impl Tuple {
    pub fn new(items: Vec<Value>) -> Tuple {
        Tuple {
            items: Rc::from(items),
        }
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        Some(self.items.get(index)?.clone())
    }

    pub fn get_slice(&self, a: usize, b: usize) -> Option<Tuple> {
        Some(Tuple::new(self.items.get(a..b)?.to_vec()))
    }
}

impl Identity for Tuple {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items).cast::<Value>() as usize
    }
}
