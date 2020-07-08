use super::{CallStack, OpAction, OpError, Operation};

new_op_empty!(StackCopy);
impl Operation for StackCopy {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = m.pop()?;
        m.push(val.clone());
        m.push(val);
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct StackLoad {
        local: u8,
    }
}

impl Operation for StackLoad {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = m.load(self.local)?.clone();
        m.push(val);
        Ok(OpAction::None)
    }
}

new_op! {
    pub struct StackStore {
        local: u8,
    }
}

impl Operation for StackStore {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = m.pop()?;
        m.store(self.local, val);
        Ok(OpAction::None)
    }
}
