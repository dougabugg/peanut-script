#[macro_use]
mod macros;

mod stackargs;

mod buffer;
mod call;
mod cmp;
mod int;
mod jump;
mod list;
mod literal;
mod num;
mod real;
mod record;
mod seq;
mod table;
mod tuple;

use super::{BytesIO, BytesReadError, DataIO};

use crate::runtime::CallFrame;

use crate::datamodel::{Function, Value, ValueTryIntoError};

use stackargs::StackArgs;

pub use buffer::{BufferCreate, BufferGetSlice, BufferSetSlice};
pub use call::{Call, Return};
pub use cmp::Cmp;
pub use int::{And, Not, Or, Shl, Shr, Xor};
pub use jump::{Jump, JumpNeg, JumpZero};
pub use list::{ListCreate, ListGetSlice, ListPop, ListPush};
pub use literal::{LiteralCreate, LiteralValue, LocalCopy};
pub use num::{Add, Div, Mul, Neg, Rem, Sub};
pub use real::{Ceil, Floor, Round, Trunc};
pub use record::{RecordCreate, RecordFromList, RecordWeakRef, WeakRecordUpgrade};
pub use seq::{SeqGet, SeqLen, SeqQuickGet, SeqQuickSet, SeqResize, SeqSet};
pub use table::TableCreate;
pub use tuple::{TupleCreate, TupleFromList};

pub trait Operation {
    fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError>;
}

pub enum OpAction {
    None,
    Jump(usize),
    Call(Function, Vec<Value>),
    Return(Value),
}

pub enum OpError {
    StackRead(usize),
    StackWrite(usize),
    IndexRead(usize),
    IndexWrite(usize),
    IntoType(ValueTryIntoError),
    BadType(&'static str),
}

impl From<ValueTryIntoError> for OpError {
    fn from(t: ValueTryIntoError) -> OpError {
        OpError::IntoType(t)
    }
}

macro_rules! create_op_type {
    ($($op:ident),+) => {
        #[repr(u8)]
        pub enum OpType {
            $($op),+
        }

        pub enum Op {
            $($op($op)),+
        }

        impl Op {
            pub fn get_type(&self) -> OpType {
                match self {
                    $(
                        Op::$op(_) => OpType::$op
                    ),+
                }
            }
        }

        impl Operation for Op {
            fn exec<'a>(&self, m: &mut CallFrame<'a>) -> Result<OpAction, OpError> {
                match self {
                    $(
                        Op::$op(op) => Operation::exec(op, m)// op.exec(m)
                    ),+
                }
            }
        }

        impl BytesIO for Op {
            #![allow(non_upper_case_globals)]
            fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
                let (b2, n) = <u8 as BytesIO>::read(b)?;
                $(
                    const $op: u8 = OpType::$op as u8;
                )+
                match n {
                    $(
                        $op => {
                            let (b, op) = <$op as BytesIO>::read(b2)?;
                            Ok( (b, Op::$op(op)) )
                        }
                    ),+
                    _ => return Err(BytesReadError::InvalidValue(b))
                }
            }
            fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
                match t {
                    $(
                        Op::$op(op) => {
                            let b = <u8 as BytesIO>::write(&(OpType::$op as u8), b)?;
                            <$op as BytesIO>::write(op, b)
                        }
                    ),+
                }
            }
        }

        $(
            impl From<$op> for Op {
                fn from(t: $op) -> Self {
                    Op::$op(t)
                }
            }
        )+
    };
}

#[rustfmt::skip]
create_op_type!(
    // num
    Add, Sub, Mul, Div, Rem, Neg,
    // int
    Shl, Shr, And, Or, Xor, Not,
    // cmp and real
    Cmp, Floor, Ceil, Trunc, Round,
    // call and jump
    Call, Return, Jump, JumpZero, JumpNeg,
    // literal
    LocalCopy, LiteralCreate,
    // tuple
    TupleCreate, TupleFromList,
    // record
    RecordCreate, RecordFromList, RecordWeakRef, WeakRecordUpgrade,
    // table
    TableCreate,
    // list
    ListCreate, ListPush, ListPop, ListGetSlice,
    // buffer
    BufferCreate, BufferGetSlice, BufferSetSlice,
    // seq
    SeqLen, SeqResize, SeqGet, SeqSet, SeqQuickGet, SeqQuickSet
);
