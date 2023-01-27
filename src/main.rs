use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use rumasm::parser::parse;

fn main(){
    let input = args().nth(1);
    let output = args().nth(2);
    
    if input == None {
        println!("Usage:");
        println!("   rumasm [INPUT_FILE]");
        println!("   rumasm [INPUT_FILE] [OUTPUT_FILE]");
        std::process::exit(0)
    }
    
    let buffer_file = File::open(input.unwrap());
    let file;
    match buffer_file{
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0)
        }
    }
    
    let out_name = match output{
        Some(s) => {s},
        None => "out.bin".to_string()
    };
    
    let mut instructions = Vec::new();
    
    let mut line_number = 1;
    for line in BufReader::new(file).lines(){
        let out = parse(line.unwrap());
        
        if out.is_err() {
            let e = out.err().unwrap();
            eprintln!("{} Line number: {}", e, line_number);
            println!("Assembly failed.");
            std::process::exit(0);
        }
        
        let p = out.ok().unwrap();
        
        match p{
            None => {}
            Some(inst) =>{
                let a: [u8; 4] = inst.to_be_bytes();
                instructions.push(a);
            }
        }
        
        line_number += 1;
    }
    
    // create an OpenOptions that allows for file creation, writing, and overwriting.
    let mut foo = OpenOptions::new().create(true).write(true).truncate(true).open(out_name).unwrap();
    
    // not sure how to do this better.
    for word in instructions{
        foo.write_all(&word).unwrap();
    }
    
    println!("Assembly completed. Output file");
}