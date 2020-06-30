use std::any::Any;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::rc::Rc;

use super::{Buffer, Function, List, Record, Table, Tuple, WeakRecord};

pub type Bool = bool;
pub type Integer = i64;
pub type Real = f64;
pub type Unknown = Rc<dyn Any>;

pub type NativeFn = fn(Vec<Value>) -> Value;

#[derive(Clone)]
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
    NativeFn(NativeFn),
    Unknown(Unknown),
}

impl Value {
    pub fn get_inner_type_name(&self) -> &'static str {
        match self {
            Value::None => "None",
            Value::Bool(_) => "Bool",
            Value::Integer(_) => "Integer",
            Value::Real(_) => "Real",
            Value::Tuple(_) => "Tuple",
            Value::Record(_) => "Record",
            Value::WeakRecord(_) => "WeakRecord",
            Value::Table(_) => "Table",
            Value::List(_) => "List",
            Value::Buffer(_) => "Buffer",
            Value::Function(_) => "Function",
            Value::NativeFn(_) => "NativeFn",
            Value::Unknown(_) => "Unknown",
        }
    }

    pub fn get_inner_type(&self) -> i64 {
        match self {
            Value::None => 0,
            Value::Bool(_) => 1,
            Value::Integer(_) => 2,
            Value::Real(_) => 3,
            Value::Tuple(_) => 4,
            Value::Record(_) => 5,
            Value::WeakRecord(_) => 6,
            Value::Table(_) => 7,
            Value::List(_) => 8,
            Value::Buffer(_) => 9,
            Value::Function(_) => 10,
            Value::NativeFn(_) => 11,
            Value::Unknown(_) => 12,
        }
    }
}

pub struct ValueTryIntoError {
    pub found: &'static str,
    pub expected: &'static str,
}

pub trait Identity {
    fn identity(&self) -> usize;
}

impl Identity for Unknown {
    fn identity(&self) -> usize {
        Rc::as_ptr(self).cast::<()>() as usize
    }
}

impl From<Ordering> for Value {
    fn from(t: Ordering) -> Self {
        Value::Integer(match t {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        })
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(t: Option<T>) -> Self {
        match t {
            Some(t) => t.into(),
            None => Value::None,
        }
    }
}

macro_rules! enum_impl_conversion {
    ($t:ident) => {
        impl From< $t > for Value {
            fn from(t: $t ) -> Self {
                Value:: $t (t)
            }
        }

        impl TryInto< $t > for Value {
            type Error = ValueTryIntoError;
            fn try_into(self) -> Result<$t, Self::Error> {
                match self {
                    Value :: $t (t) => Ok(t),
                    _ => Err(ValueTryIntoError {
                        found: self.get_inner_type_name(),
                        expected: stringify!($t),
                    }),
                }
            }
        }

        impl<'a> TryInto< &'a $t > for &'a Value {
            type Error = ValueTryIntoError;
            fn try_into(self) -> Result<&'a $t, Self::Error> {
                match self {
                    Value :: $t (t) => Ok(t),
                    _ => Err(ValueTryIntoError {
                        found: self.get_inner_type_name(),
                        expected: stringify!($t),
                    }),
                }
            }
        }
    };
    ($($t:ident),+) => {
        $( enum_impl_conversion!($t); )+
    }
}

enum_impl_conversion!(
    Bool, Integer, Real, Tuple, Record, WeakRecord, Table, List, Buffer, Function, NativeFn,
    Unknown
);
