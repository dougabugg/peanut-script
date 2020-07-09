use std::collections::BTreeMap;

use super::Op;

struct Label {
    target: Option<usize>,
    pub jumps: Vec<usize>,
}

impl Label {
    pub fn new() -> Label {
        Label {
            target: None,
            jumps: Vec::new(),
        }
    }

    pub fn set_target(&mut self, target: usize) {
        match self.target {
            Some(_) => panic!("label target already set"),
            None => self.target = Some(target),
        }
    }

    pub fn get_target(&self) -> usize {
        self.target.expect("label target not set")
    }
}

pub struct CodeGenerator {
    ops: Vec<Op>,
    labels: BTreeMap<usize, Label>,
    counter: usize,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            ops: Vec::new(),
            labels: BTreeMap::new(),
            counter: usize::MAX,
        }
    }

    pub fn append(&mut self, mut ops: Vec<Op>) {
        self.ops.append(&mut ops);
    }

    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }

    fn get_label(&mut self, label: usize) -> &mut Label {
        match self.labels.get_mut(&label) {
            Some(l) => l,
            None => panic!("label with id {} not found", label),
        }
    }

    pub fn register_label(&mut self, i: usize) {
        let conflict = self.labels.insert(i, Label::new());
        match conflict {
            Some(_) => panic!("label with id {} already exists", i),
            None => {}
        }
    }

    pub fn create_label(&mut self) -> usize {
        let i = self.counter;
        self.counter -= 1;
        self.register_label(i);
        i
    }

    pub fn label_here(&mut self, label: usize) {
        let target = self.ops.len();
        self.get_label(label).set_target(target);
    }

    pub fn push_jump(&mut self, label: usize, jump: Op) {
        let i = self.ops.len();
        self.get_label(label).jumps.push(i);
        match jump {
            Op::Jump(_) | Op::JumpZero(_) | Op::JumpNeg(_) => {}
            _ => panic!("expected jump op, but found {} op", jump.get_type().get_name()),
        }
        self.ops.push(jump);
    }

    pub fn into_vec(self) -> Vec<Op> {
        let mut ops = self.ops;
        for label in self.labels.values() {
            let target = label.get_target() as i32;
            for &jump in &label.jumps {
                let target = target - jump as i32;
                match &mut ops[jump] {
                    Op::Jump(j) => j.dest = target,
                    Op::JumpZero(j) => j.dest = target,
                    Op::JumpNeg(j) => j.dest = target,
                    _ => unreachable!(),
                }
            }
        }
        ops
    }
}
