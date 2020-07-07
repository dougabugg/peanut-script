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
    labels: Vec<Label>,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            ops: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn here(&self) -> usize {
        self.ops.len()
    }

    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }

    pub fn create_label(&mut self) -> usize {
        let i = self.labels.len();
        self.labels.push(Label::new())
        i
    }

    pub fn label_target(&mut self, label: usize, target: usize) {
        self.labels[label].set_target(target);
    }

    pub fn label_here(&mut self, label: usize) {
        let target = self.ops.len();
        self.labels[label].set_target(target);
    }

    pub fn push_jump(&mut self, label: usize, jump: Op) {
        let i = self.ops.len();
        self.labels[label].jumps.push(i);
        match jump {
            Op::Jump(_) | Op::JumpZero(_) | Op::JumpNeg(_) => {},
            _ => panic!("op supplied is not a jump"),
        }
        self.ops.push(jump);
    }

    pub fn into_ops(self) -> Vec<Op> {
        for label in &self.labels {
            let target = label.get_target();
            for jump in &label.jumps {
                match &mut self.ops[jump] {
                    Op::Jump(j) => j.dest = target,
                    Op::JumpZero(j) => j.dest = target,
                    Op::JumpNeg(j) => j.dest = target,
                    _ => unreachable!(),
                }
            }
        }
        self.ops
    }
}
