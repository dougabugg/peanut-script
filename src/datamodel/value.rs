use std::rc::Rc;
use std::any::Any;

// Bool, Identity, Integer, Real, Unknown, Value, ValueTryIntoError

pub type Bool = bool;
pub type Integer = i64;
pub type Real = f64;
pub type Unknown = Rc<dyn Any>;

pub enum Value {
    None,
    Bool(Bool),
    Integer(Integer),
    Real(Real),
    Tuple(Tuple),
    Record(Record),
    WeakRecord(WeakRecord),
    Table(Table),
    List(List),
    Buffer(Buffer),
    Function(Function),
    Unknown(Unknown),
}

pub trait Identity {
    fn identity(&self) -> usize;
}

