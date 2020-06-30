use crate::bytecode::OpError;
use crate::datamodel::Function;

use super::CallFrame;

pub struct VirtualMachine {
    frame: Box<CallFrame>,
}

impl VirtualMachine {
    pub fn new(func: Function) -> VirtualMachine {
        VirtualMachine {
            frame: Box::new(CallFrame::new(func))
        }
    }

    pub fn step(&mut self) -> Result<(), OpError> {
        // TODO stopped right here
        panic!();
    }
}
