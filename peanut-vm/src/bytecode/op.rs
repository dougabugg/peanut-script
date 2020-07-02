use super::{BytesIO, BytesReadError};

use crate::CallStack;

use crate::datamodel::{Function, NativeFn, Value, ValueTryIntoError};

use super::ops::*;

pub trait Operation {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError>;
}

pub enum OpAction {
    None,
    Jump(usize),
    Call(Function, Vec<Value>, u8),
    CallNative(NativeFn, Vec<Value>, u8),
    Return(Value),
}

pub enum OpError {
    StackRead(u8),
    StackWrite(u8),
    IndexRead(i64),
    IndexWrite(i64),
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
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                match self {
                    $(
                        Op::$op(op) => op.exec(m)
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
    TupleCreate, TupleFromList, TupleWeakRef, TupleWeakUpgrade,
    // table and list
    TableCreate, ListCreate, ListPush, ListPop, ListGetSlice,
    // buffer
    BufferCreate, BufferGetSlice, BufferSetSlice,
    // seq
    SeqLen, SeqResize, SeqGet, SeqSet, SeqQuickGet, SeqQuickSet, SeqToList, SeqAppend
);
