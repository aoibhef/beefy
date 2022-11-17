use num_traits::FromPrimitive;

use crate::{fungespace::FungeSpace, instructions::Instruction};

pub struct InstructionPointer {
    pub pos: (i64, i64),
    pub delta: (i64, i64),
    pub offset: (i64, i64),
    pub alive: bool,
    pub stringmode: bool,
}

impl InstructionPointer {
    pub fn new() -> InstructionPointer {
        InstructionPointer {
            pos: (0, 0),
            delta: (1, 0),
            offset: (0, 0),
            alive: true,
            stringmode: false,
        }
    }

    fn step(&mut self) {
        self.pos.0 += self.delta.0;
        self.pos.1 += self.delta.1;
    }

    fn step_backwards(&mut self) {
        self.pos.0 -= self.delta.0;
        self.pos.1 -= self.delta.1;
    }

    pub fn step_wrapped(&mut self, fs: &FungeSpace) {
        self.step();

        if !fs.in_bounds(&self) {
            self.step_backwards();
            while fs.in_bounds(&self) {
                self.step_backwards();
            }

            self.step();
            assert!(fs.in_bounds(&self));
        }
    }

    pub fn step_skip_spaces_and_comments(&mut self, fs: &FungeSpace) {
        let mut in_comment = false;
        loop {
            let i = fs.instruction_at_ip(&self);
            match FromPrimitive::from_i64(i) {
                Some(Instruction::Space) => self.step_wrapped(&fs),
                Some(Instruction::JumpOver) => {
                    in_comment = !in_comment;
                    self.step_wrapped(&fs);
                },
                _ => {
                    if in_comment {
                        self.step_wrapped(&fs);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn move_to_next_instruction(&mut self, fs: &FungeSpace) {
        self.step_wrapped(&fs);
        self.step_skip_spaces_and_comments(&fs);
    }

    pub fn reflect(&mut self) {
        self.delta.0 = -self.delta.0;
        self.delta.1 = -self.delta.1;
    }
}
