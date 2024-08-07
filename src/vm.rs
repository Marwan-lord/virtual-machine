use crate::memory::*;

pub enum Register {
    A,
    B,
    C,
    M,
    Sp,
    Pc,
    Bp,
    Flags,
}

pub struct Machine {
    register: [u16; 8],
    memory: Box<dyn Addressable> ,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            register: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.register[Register::Pc as usize];
        let instruction = self.memory.read2(pc).unwrap();
        println!("{} @ {}", instruction, pc);
        Ok(())
    }
}
