use crate::fungespace::FungeSpace;
use crate::instruction_pointer::InstructionPointer;

mod fungespace;
mod instruction_pointer;

fn main() {
    let mut fs = FungeSpace::from_file("prog/sanity.bf");
    let mut ip = InstructionPointer::new();

    loop {
        let i = fs.instruction_at_ip(&ip);
        match i {
            48 /* 0 */ => todo!(),
            49 /* 1 */ => todo!(),
            50 /* 2 */ => todo!(),
            51 /* 3 */ => todo!(),
            52 /* 4 */ => todo!(),
            53 /* 5 */ => todo!(),
            54 /* 6 */ => todo!(),
            55 /* 7 */ => todo!(),
            56 /* 8 */ => todo!(),
            57 /* 9 */ => todo!(),
            _ => ip.reflect(),
        }
    }
}