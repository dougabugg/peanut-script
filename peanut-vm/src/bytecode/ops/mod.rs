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
mod seq;
mod table;
mod tuple;

use super::{BytesIO, BytesReadError, DataIO, OpAction, OpError, Operation};

use crate::CallStack;

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
pub use seq::{SeqAppend, SeqGet, SeqLen, SeqQuickGet, SeqQuickSet, SeqResize, SeqSet, SeqToList};
pub use table::{TableCreate, TableGet, TableSet};
pub use tuple::{TupleCreate, TupleFromList, TupleWeakRef, TupleWeakUpgrade};
