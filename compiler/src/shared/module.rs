use super::LiteralValue;

pub enum ModuleItem<F> {
    LiteralValue(LiteralValue),
    Buffer(Vec<u8>),
    ModuleRef(u32),
    Function(F),
}

pub struct Module<F> {
    pub items: Vec<ModuleItem<F>>,
}

pub struct Program<F> {
    pub modules: Vec<Module<F>>,
}
