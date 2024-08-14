use std::{env, fs::File, io::{self, BufRead, Write}, path::Path};

fn main() -> Result<(), String>{
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x|format!("failed to open: {}", x))?; 
    
    let mut output: Vec<u8> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_inner = line.map_err(|_x| "foo")?;
        for token in line_inner.split(" ").filter(|x| x.len() > 0) {
            let b = u8::from_str_radix(token, 16).map_err(|x| format!("parse int: {}", x))?;
            output.push(b)
        }
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(|x| format!("{}", x))?;
    Ok(())
}
