use std::env;
use std::fs::File;
use std::io::prelude::*;

pub mod types;
pub mod parser;
pub mod compiler;

use parser::*;
use compiler::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut contents = String::new();
    in_file.read_to_string(&mut contents)?;

    let expr = parse(&contents);
    let asm_program = compile(&expr);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
