use crate::memory::*;

#[derive(Debug)]
#[repr(u8)]
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

impl Register {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::Sp as u8 => Some(Register::Sp),
            x if x == Register::Pc as u8 => Some(Register::Pc),
            x if x == Register::Bp as u8 => Some(Register::Bp),
            x if x == Register::Flags as u8 => Some(Register::Flags),
            _ => None,
        }
    }
}
#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
}

impl Op {
    pub fn value(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

// instruction = [ 0 0 0 0 0 0 0 0 0 0 |  0 0 0 0 0 0 0 0 0 ]
//                       OPERATOR      | ARG(S)
//                                     | 8 bit literal
//                                     | REG1 | REG2

fn parse_instruction(ins: u16) -> Result<Op, String> {
    let op = (ins & 0xff) as u8;
    match op {
        x if x == Op::Nop.value() => Ok(Op::Nop),
        
        x if x == Op::Push(0).value() => {
            let arg = (ins & 0xff00) >> 8;
            Ok(Op::Push(arg as u8))
        }

        x if x == Op::PopRegister(Register::A).value() => {
            let reg = (ins & 0xf00) >> 8;
            if let Some(r) = Register::from_u8(reg as u8) {
                Ok(Op::PopRegister(r))
            } else {
                Err(format!("unkown register: 0x{:X}", reg))
            }
        }

        x if x == Op::AddStack.value() => Ok(Op::AddStack),
        _ => Err(format!("unknown operation 0x{:X}", op)),
    }
}

pub struct Machine {
    register: [u16; 8],
    pub memory: Box<dyn Addressable>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            register: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.register[r as usize]
    }

    pub fn pop(&mut self) -> Result<u16, String> {
        let sp = self.register[Register::Sp as usize] - 2;
        if let Some(v) = self.memory.read2(sp) {
            self.register[Register::Sp as usize] -= 2;
            Ok(v)
        } else {
            Err(format!("memory read fault @ 0x{:X}", sp))
        }
    }

    pub fn push(&mut self, v: u16) -> Result<(), String> {
        let sp = self.register[Register::Sp as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault @ 0x{:X}", sp));
        }
        self.register[Register::Sp as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.register[Register::Pc as usize];
        let instruction = self.memory.read2(pc).unwrap();

        self.register[Register::Pc as usize] = pc + 2;
        let op = parse_instruction(instruction)?;

        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => self.push(v.into()),
            Op::PopRegister(r) => {
                let value = self.pop()?;
                self.register[r as usize] = value;
                Ok(())
            }
            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Op::AddRegister(r1, r2) => {
                self.register[r1 as usize] += self.register[r2 as usize];
                Ok(())
            }
        }
    }
}
