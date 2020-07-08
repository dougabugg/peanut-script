use crate::vm::bytecode::{Op, ops};

mod codegen;
mod expr;

use codegen::CodeGenerator;

pub trait Compile {
    fn compile(&self) -> Vec<Op>;
}