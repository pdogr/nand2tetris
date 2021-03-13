#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod asm;
use crate::asm::assembler::Assembler;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let (input, output) = match args.len() {
        3 => (&args[1], &args[2]),
        _ => {
            println!("usage: cargo run <input_file> <output_file>");
            std::process::exit(1);
        }
    };

    let mut file = std::fs::File::open(input)?;
    let mut out = std::fs::File::create(output)?;
    let mut a = Assembler::new(&mut file);
    a.assemble(&mut out);
    Ok(())
}
