pub struct InstructionPointer {
    pub pos: (i64, i64),
    pub delta: (i64, i64),
    pub offset: (i64, i64),
    pub alive: bool,
}

impl InstructionPointer {
    pub fn new() -> InstructionPointer {
        InstructionPointer {
            pos: (0, 0),
            delta: (1, 0),
            offset: (0, 0),
            alive: true,
        }
    }

    pub fn reflect(&mut self) {
        self.delta.0 = -self.delta.0;
        self.delta.1 = -self.delta.1;
    }
}
