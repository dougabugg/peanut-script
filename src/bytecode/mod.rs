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

use crate::runtime::CallFrame;

use stackargs::StackArgs;

use call::{Call, Return};
use cmp::Cmp;
use int::{And, Not, Or, Shl, Shr, Xor};
pub use io::{BytesIO, BytesReadError, DataIO};
use jump::{Jump, JumpNeg, JumpZero};
use literal::{LiteralCreate, LiteralValue, LocalCopy};
use num::{Add, Div, Mul, Neg, Rem, Sub};
use op::{OpAction, OpError, Operation};
use real::{Ceil, Floor, Round, Trunc};
use record::{RecordCreate, RecordFromList, RecordWeakRef};
use table::TableCreate;

create_op_type!(
    // num
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    // int
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Not,
    // cmp and real
    Cmp,
    Floor,
    Ceil,
    Trunc,
    Round,
    // call and jump
    Call,
    Return,
    Jump,
    JumpZero,
    JumpNeg,
    // literal
    LocalCopy,
    LiteralCreate,
    // record
    RecordCreate,
    RecordFromList,
    RecordWeakRef,
    TableCreate
);
