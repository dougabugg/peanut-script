use std::any::Any;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::rc::Rc;

use super::{Buffer, Function, List, Table, Tuple, TupleWeak};

pub type Integer = i64;
pub type Real = f64;
pub type Unknown = Rc<dyn Any>;

pub type NativeFn = fn(Vec<Value>) -> Value;

macro_rules! create_value_enum {
    ($($n:ident),+) => {
        #[derive(Clone)]
        pub enum Value {
            None,
            $($n($n)),+
        }

        #[repr(u8)]
        #[derive(PartialEq)]
        pub enum ValueType {
            None,
            $($n),+
        }

        impl Value {
            pub fn get_inner_type_name(&self) -> &'static str {
                match self {
                    Value::None => "None",
                    $(Value::$n(_) => stringify!($n)),+
                }
            }

            pub fn get_inner_type(&self) -> ValueType {
                match self {
                    Value::None => ValueType::None,
                    $(Value::$n(_) => ValueType::$n),+
                }
            }
        }

        $(create_value_enum!(conversion $n);)+
    };
    (conversion $t:ident) => {
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
}

create_value_enum! {
    Integer, Real, Tuple, TupleWeak, Table, List, Buffer, Function, NativeFn, Unknown
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
