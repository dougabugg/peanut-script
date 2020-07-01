use std::mem;

use crate::bytecode::{OpAction, OpError};
use crate::datamodel::{Function, Value};

use super::CallFrame;

pub struct VirtualMachine {
    frame: Option<Box<CallFrame>>,
}

impl VirtualMachine {
    pub fn new(func: Function) -> VirtualMachine {
        VirtualMachine {
            frame: Some(Box::new(CallFrame::new(func))),
        }
    }

    pub fn run_until_exited(&mut self) -> Result<Value, OpError> {
        loop {
            let action = self.step()?;
            match self.process(action)? {
                VmState::Running => continue,
                VmState::Exited(val) => return Ok(val),
            }
        }
    }

    pub fn step(&mut self) -> Result<OpAction, OpError> {
        let frame = self.frame.as_mut().unwrap();
        frame.exec()
    }

    pub fn process(&mut self, action: OpAction) -> Result<VmState, OpError> {
        match action {
            OpAction::None => (),
            OpAction::Jump(dest) => {
                let frame = self.frame.as_mut().unwrap();
                frame.jump(dest);
            }
            OpAction::Call(func, args, output) => {
                let mut callee = Box::new(CallFrame::new(func));
                callee.output = output;
                for (i, arg) in args.into_iter().enumerate() {
                    callee.store(i as u8, arg)?;
                }
                mem::swap(&mut self.frame, &mut callee.parent);
                self.frame = Some(callee);
            }
            OpAction::CallNative(func, args, output) => {
                let frame = self.frame.as_mut().unwrap();
                frame.store(output, func(args))?;
            }
            OpAction::Return(val) => {
                let frame = self.frame.as_mut().unwrap();
                let mut parent = None;
                mem::swap(&mut frame.parent, &mut parent);
                let output = frame.output;
                match parent {
                    Some(mut parent) => {
                        parent.store(output, val)?;
                        self.frame = Some(parent);
                    }
                    None => {
                        self.frame = None;
                        return Ok(VmState::Exited(val));
                    }
                }
            }
        }
        Ok(VmState::Running)
    }
}

pub enum VmState {
    Running,
    Exited(Value),
}
