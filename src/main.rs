mod error;
mod parser;
mod renderer;
mod unicode;
mod string;
use renderer::render;
use crate::error::*;
use crate::parser::*;
use std::env;

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
    if input.len() > 4 {
        print!("Usage: repc [char]");
        return Err(RepcError);
    }
    let rep = decode(input)?;
    render(&rep);
    Ok(())
}
