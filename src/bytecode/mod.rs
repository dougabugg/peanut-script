#[macro_use]
mod macros;

mod stackargs;

mod io;
#[macro_use]
mod op;

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

use crate::runtime::CallFrame;

pub use io::{BytesIO, BytesReadError, DataIO};
pub use op::{Op, OpAction, OpError, OpType, Operation};
use stackargs::StackArgs;

pub use buffer::{BufferCreate, BufferGetSlice, BufferSetSlice};
pub use call::{Call, Return};
pub use cmp::Cmp;
pub use int::{And, Not, Or, Shl, Shr, Xor};
pub use jump::{Jump, JumpNeg, JumpZero};
pub use list::{ListCreate, ListGetSlice, ListPop, ListPush};
pub use literal::{LiteralCreate, LocalCopy};
pub use num::{Add, Div, Mul, Neg, Rem, Sub};
pub use real::{Ceil, Floor, Round, Trunc};
pub use record::{RecordCreate, RecordFromList, RecordWeakRef, WeakRecordUpgrade};
pub use seq::{SeqGet, SeqLen, SeqQuickGet, SeqQuickSet, SeqResize, SeqSet};
pub use table::TableCreate;
pub use tuple::{TupleCreate, TupleFromList};
