use std::{
    env,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

use virtual_machine::{op::Instruction, op::OpCode, register::Register};

#[derive(Debug)]
pub enum AsmErr {
    ParseErr,
    UnknownReg,
    UnknownOpCode,
    LenErr,
    NoArg,
    UnknownFile,
    WriteErr,
}

fn parse_numeric(s: &str) -> Result<u8, AsmErr> {
    if s.is_empty() {
        return Err(AsmErr::ParseErr);
    }
    let fst = s.chars().next().unwrap();
    let (num, radix) = match fst {
        '$' => (&s[1..], 16),
        '%' => (&s[1..], 2),
        _ => (s, 10),
    };
    u8::from_str_radix(num, radix).map_err(|_| AsmErr::ParseErr)
}

fn parse_register(s: &str) -> Result<Register, AsmErr> {
    match s {
        "A" => Ok(Register::A),
        _ => Err(AsmErr::UnknownReg),
    }
}

fn assert_length(parts: &[&str], n: usize) -> Result<(), AsmErr> {
    if parts.len() == n {
        Ok(())
    } else {
        Err(AsmErr::LenErr)
    }
}

fn handle_line(parts: &[&str]) -> Result<Instruction, AsmErr> {
    let opcode = OpCode::from_string(parts[0]).ok_or(AsmErr::UnknownOpCode)?;
    match opcode {
        OpCode::Nop => Ok(Instruction::Nop),
        OpCode::Push => {
            assert_length(parts, 2)?;
            Ok(Instruction::Push(parse_numeric(parts[1])?))
        }
        OpCode::AddStack => {
            assert_length(parts, 1)?;
            Ok(Instruction::AddStack)
        }
        OpCode::AddRegister => {
            assert_length(parts, 3)?;
            Ok(Instruction::AddRegister(
                parse_register(parts[1])?,
                parse_register(parts[2])?,
            ))
        }
        OpCode::PopRegister => {
            assert_length(parts, 2)?;
            Ok(Instruction::PopRegister(parse_register(parts[1])?))
        }
        OpCode::PushRegister => {
            assert_length(parts, 2)?;
            Ok(Instruction::PushRegister(parse_register(parts[1])?))
        }
        OpCode::Signal => {
            assert_length(parts, 2)?;
            Ok(Instruction::Signal(parse_numeric(parts[1])?))
        }
    }
}

fn main() -> Result<(), AsmErr> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <assembly file>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|_| AsmErr::UnknownFile)?;
    let mut output: Vec<u8> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line_inner = line.expect("line error");

        if line_inner.is_empty() {
            continue;
        }
        if line_inner.starts_with(';') {
            continue;
        }
        let parts: Vec<_> = line_inner.split(' ').filter(|x| !x.is_empty()).collect();

        if parts.is_empty() {
            continue;
        }

        let instruction = handle_line(&parts)?;
        let raw_instruction: u16 = instruction.encode_u16();
        output.push((raw_instruction & 0xff) as u8);
        output.push((raw_instruction >> 8) as u8);
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(|_| AsmErr::WriteErr)?;
    Ok(())
}
