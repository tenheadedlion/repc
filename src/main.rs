mod error;
mod parser;
mod renderer;
use std::env;
use crate::error::*;
use crate::parser::*;
use crate::renderer::*;

fn main() -> Result<(), RepcError> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print!("Usage: repc [char]");
        return Err(RepcError);
    }
    let input = &args[1];
    if input.eq("-h") || input.eq("--help") {
        print!("Usage: repc [char]");
        return Err(RepcError);
    }
    let rep = decode(input)?;
    print!("{}", rep);
    Ok(())
}
