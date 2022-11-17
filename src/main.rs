use num_traits::FromPrimitive;

use crate::fungespace::FungeSpace;
use crate::instruction_pointer::InstructionPointer;
use crate::instructions::Instruction;
use crate::stackstack::StackStack;

mod fungespace;
mod instruction_pointer;
mod instructions;
mod stackstack;

fn main() {
    let mut fs = FungeSpace::from_file("prog/sanity.bf");
    let mut ip = InstructionPointer::new();
    let mut ss = StackStack::new();

    // In case the loaded file starts with a comment or whitespace
    ip.step_skip_spaces_and_comments(&fs);

    loop {
        let i = fs.instruction_at_ip(&ip);

        match FromPrimitive::from_i64(i) {
            Some(Instruction::Space) => unreachable!(),
            Some(Instruction::Trampoline) => ip.step_wrapped(&fs),
            Some(Instruction::OutputInteger) => print!("{} ", ss.pop()),
            Some(Instruction::PushZero) => ss.push(0),
            Some(Instruction::PushOne) => ss.push(1),
            Some(Instruction::PushTwo) => ss.push(2),
            Some(Instruction::PushThree) => ss.push(3),
            Some(Instruction::PushFour) => ss.push(4),
            Some(Instruction::PushFive) => ss.push(5),
            Some(Instruction::PushSix) => ss.push(6),
            Some(Instruction::PushSeven) => ss.push(7),
            Some(Instruction::PushEight) => ss.push(8),
            Some(Instruction::PushNiner) => ss.push(9),
            Some(Instruction::Stop) => break,
            None => ip.reflect(),
            _ => ip.reflect()
        }

        ip.move_to_next_instruction(&fs);
    }
}