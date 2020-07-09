use crate::vm::bytecode::{ops, Op};

mod codegen;

use codegen::CodeGenerator;

mod binaryop;
mod expr;
mod statement;
mod unaryop;

pub use binaryop::{BinaryOp, BinaryOpType};
pub use expr::{Expr, Literal};
pub use unaryop::{UnaryOp, UnaryOpType};
pub use statement::{Statement, IfElse, If, Loop, LoopType, BufferSetSlice};

/*
Some notes on usage of stage0 AST

Code generation for the stage0 AST assumes the following:
    - the expressions Call, TupleCreate, and ListCreate each have Vec<Expr>. the vecs
        are limited to 255 expressions.
    - local variables (Expr::LocalScope) are correctly and efficiently numbered. once a
        variable isn't referenced anymore, it's number should be reused.
    - all loop labels are uniquely numbered. the labels in break and continue statements
        must match up with the labels in a parent loop statment.

some assumptions are checked at runtime and will cause a panic during compilation if
they are broken, but others cannot be checked and may silently produce incorrect bytecode
programs, so care must be taken when using stage0 for code generation.
*/
