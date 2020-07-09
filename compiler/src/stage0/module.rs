use crate::vm::bytecode;

use super::{ops, CodeGenerator, Literal, Statement};

pub struct Function {
    args: Vec<u8>,
    body: Vec<Statement>,
}

impl Function {
    pub fn compile(&self) -> bytecode::Function {
        let mut g = CodeGenerator::new();
        // order of args popped from stack is (A, B, C)
        // see vm::VirtualMachine for details
        for &arg in &self.args {
            g.push(ops::StackStore::new(arg).into());
        }
        for statement in &self.body {
            statement.compile(&mut g);
        }
        bytecode::Function { ops: g.into_vec() }
    }
}

pub enum ModuleItem {
    Literal(Literal),
    Buffer(Vec<u8>),
    ModuleRef(u32),
    Function(Function),
}

impl ModuleItem {
    pub fn compile(self) -> bytecode::ModuleItem {
        match self {
            ModuleItem::Literal(l) => bytecode::ModuleItem::LiteralValue(l),
            ModuleItem::Buffer(b) => bytecode::ModuleItem::Buffer(b),
            ModuleItem::ModuleRef(i) => bytecode::ModuleItem::ModuleRef(i),
            ModuleItem::Function(f) => bytecode::ModuleItem::Function(f.compile()),
        }
    }
}

pub struct Module {
    items: Vec<ModuleItem>,
}

impl Module {
    pub fn compile(self) -> bytecode::Module {
        bytecode::Module { items: self.items.into_iter().map(|m| m.compile()).collect() }
    }
}

pub struct Program {
    modules: Vec<Module>,
}

impl Program {
    pub fn compile(self) -> bytecode::Program {
        bytecode::Program { modules: self.modules.into_iter().map(|m| m.compile()).collect() }
    }
}
