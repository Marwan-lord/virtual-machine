use std::collections::HashMap;

use crate::memory::*;
use crate::op::*;
use crate::register::*;

#[derive(Debug)]
pub enum MachineErr {
    UnknownOp,
    UnknownReg,
    ReadFault,
    WriteFault,
    PcFault,
    UnknownSignal,
    UnknownFile,

}

fn parse_instruction_arg(ins: u16) -> u8 {
    ((ins & 0xff00) >> 8) as u8
}

fn parse_instruction(ins: u16) -> Result<Instruction, MachineErr> {
    let op = (ins & 0xff) as u8;
    match OpCode::from_u8(op).ok_or(MachineErr::UnknownOp)? {
        OpCode::Nop => Ok(Instruction::Nop),

        OpCode::Push => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Push(arg))
        }

        OpCode::PopRegister => {
            let reg = (ins & 0xf00) >> 8;
            Register::from_u8(reg as u8)
                .ok_or(MachineErr::UnknownReg)
                .map(|r| Instruction::PopRegister(r))
        }

        OpCode::PushRegister => {
            let reg = (ins & 0xf00) >> 8;
            Register::from_u8(reg as u8)
                .ok_or(MachineErr::UnknownReg)
                .map(|r| Instruction::PushRegister(r))
        }

        OpCode::AddRegister => {
            let reg1_raw = (ins & 0xf00) >> 8;
            let reg2_raw = (ins & 0xf000) >> 12;

            let reg1 = Register::from_u8(reg1_raw as u8)
                .ok_or(MachineErr::UnknownReg)?;

            let reg2 = Register::from_u8(reg2_raw as u8)
                .ok_or(MachineErr::UnknownReg)?;

            Ok(Instruction::AddRegister(reg1, reg2))
        }

        OpCode::AddStack => Ok(Instruction::AddStack),
        OpCode::Signal => {
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Signal(arg))
        }
    }
}

pub type SignalFunction = fn(&mut Machine) -> Result<(), MachineErr>;

pub struct Machine {
    register: [u16; 8],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool,
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
            signal_handlers: HashMap::new(),
            halt: false,
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }

    pub fn state(&self) -> String {
        format!(
            "A: {} | B: {} | C: {} | M: {}
SP: {} | PC: {} | BP: {}
FLAGS: {:X}",
            self.get_register(Register::A),
            self.get_register(Register::B),
            self.get_register(Register::C),
            self.get_register(Register::M),
            self.get_register(Register::Sp),
            self.get_register(Register::Pc),
            self.get_register(Register::Bp),
            self.get_register(Register::Flags),
        )
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    pub fn get_register(&self, r: Register) -> u16 {
        self.register[r as usize]
    }

    pub fn set_register(&mut self, r: Register, v: u16) {
        self.register[r as usize] = v;
    }

    pub fn pop(&mut self) -> Result<u16, MachineErr> {
        let sp = self.register[Register::Sp as usize] - 2;
        if let Some(v) = self.memory.read2(sp) {
            self.register[Register::Sp as usize] -= 2;
            Ok(v)
        } else {
            Err(MachineErr::ReadFault)
        }
    }

    pub fn push(&mut self, v: u16) -> Result<(), MachineErr> {
        let sp = self.register[Register::Sp as usize];
        if !self.memory.write2(sp, v) {
            return Err(MachineErr::WriteFault);
        }
        self.register[Register::Sp as usize] += 2;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), MachineErr> {
        let pc = self.register[Register::Pc as usize];
        let instruction = self
            .memory
            .read2(pc)
            .ok_or(MachineErr::PcFault)?;
        self.register[Register::Pc as usize] = pc + 2;
        let op = parse_instruction(instruction)?;

        match op {
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => self.push(v.into()),
            Instruction::PopRegister(r) => {
                let value = self.pop()?;
                self.register[r as usize] = value;
                Ok(())
            }
            Instruction::PushRegister(r) => {
                self.push(self.register[r as usize])?;
                Ok(())
            }
            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Instruction::AddRegister(r1, r2) => {
                self.register[r1 as usize] += self.register[r2 as usize];
                Ok(())
            }
            Instruction::Signal(signal) => {
                let sig_fn = self
                    .signal_handlers
                    .get(&signal)
                    .ok_or(MachineErr::UnknownSignal)?;
                sig_fn(self)
            }
        }
    }
}
