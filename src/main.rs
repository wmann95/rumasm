use std::collections::HashMap;
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
    
    let input = input.unwrap();
    
    let out_name = match output{
        Some(s) => {s},
        None => "out.bin".to_string()
    };
    
    let errors = linter(input.clone());
    
    if errors.len() > 0{
        eprintln!();
        for e in errors {
            eprintln!("{}", e);
        }
        eprintln!("Errors found, assembly failed.");
        eprintln!();
        std::process::exit(0);
    }
    
    let instructions = assembler(input.clone());
    
    // create an OpenOptions that allows for file creation, writing, and overwriting.
    let mut foo = OpenOptions::new().create(true).write(true).truncate(true).open(out_name.clone()).unwrap();
    
    // not sure how to do this better.
    for word in instructions{
        foo.write_all(&word).unwrap();
    }
    
    println!("Assembly completed. Output to file: {}", out_name.clone());
}

fn linter(input: String) -> Vec<String>{
    let buffer_file = File::open(input);
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
    
    let mut errors = Vec::new();

    let mut line_number = 1;
    for line in BufReader::new(file).lines(){
        let out = parse(line.unwrap());
        
        if out.is_err() {
            let e = out.err().unwrap();
            let error = format!("{} Line number: {}", e, line_number);
            errors.push(error);
        }

        line_number += 1;
    }
    
    return errors;
}

fn assembler(input: String) -> Vec<[u8; 4]>{
    let buffer_file = File::open(input);
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
    
    let mut instructions = Vec::new();
    let mut labels: HashMap<String, u32> = HashMap::new();
    
    let mut lines = Vec::new();
    
    let mut instruction_number = 0;
    for line in BufReader::new(file.try_clone().unwrap()).lines(){
        let l = line.unwrap();
        
        if l.contains(":") { // Label identifier.
            let (label, _) = l.split_at(l.find(":").unwrap());
            labels.insert(label.to_string(), instruction_number);
            continue;
        }
        
        lines.push(l);
        instruction_number += 1;
    }
    
    for i in labels{
        println!("{}, {}", i.0, i.1);
    }
    
    for line in lines{
        let out = parse(line);
        
        let p = out.ok().unwrap();

        match p{
            None => {}
            Some(inst) =>{
                let a: [u8; 4] = inst.to_be_bytes();
                instructions.push(a);
            }
        }
    }
    
    return instructions;
}