mod app;
mod parse;
mod format;

use std::fs::File;
use std::io::{Result, Read};

fn main() -> Result<()> {
    let params = app::params();
    
    println!("Using input file name: {}", params.input);
    
    let mut file = File::open(params.input)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let code_slices = parse::code_slices(&content);

    for a in code_slices {
        println!("{}", a);
        println!("==========");
    }

    Ok(())
}

