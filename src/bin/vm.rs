use virtual_machine::vm::*;

pub fn main () -> Result<(), &'static str> {
    let mut vm = Machine::new();
    vm.step()?;
    Ok(())
}
