use std::{env, fs::File, io::{BufReader, Read}, path::Path};
use virtual_machine::vm::*;

fn signal_halt(vm: &mut Machine) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("can't open: {}", x))?;
    let mut reader = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();
    reader.read_to_end(&mut program).unwrap();


    let mut vm = Machine::new();
    vm.define_handler(0xf0, signal_halt);
    vm.memory.load_from_vec(&program, 0);
    while !vm.halt {
        vm.step()?;
        vm.step()?;
        vm.step()?;
        vm.step()?;
    }
    println!("A =  {}", vm.get_register(Register::A));
    Ok(())
}
