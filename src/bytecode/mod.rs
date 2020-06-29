#[macro_use]
mod macros;

mod stackargs;

mod io;
#[macro_use]
mod op;
mod call;
mod cmp;
mod int;
mod jump;
mod literal;
mod num;
mod real;
mod record;
mod table;
mod tuple;

use crate::runtime::CallFrame;

pub use io::{BytesIO, BytesReadError, DataIO};
pub use op::{Op, OpAction, OpError, OpType, Operation};
use stackargs::StackArgs;
