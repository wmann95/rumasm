use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use rumasm::nom_parser;
use rumasm::parser::parse;
use rumasm::nom_parser::parse_line;

fn main(){
    nom_parser::parse("cat.rumasm");
}

// fn main(){
//     let input = args().nth(1);
//     let output = args().nth(2);
    
//     if input == None {
//         println!("Usage:");
//         println!("   rumasm [INPUT_FILE]");
//         println!("   rumasm [INPUT_FILE] [OUTPUT_FILE]");
//         std::process::exit(0)
//     }
    
//     let input = input.unwrap();
    
//     let out_name = match output{
//         Some(s) => {s},
//         None => "out.bin".to_string()
//     };
    
//     let errors = linter(input.clone());
    
//     if errors.len() > 0{
//         eprintln!();
//         for e in errors {
//             eprintln!("{}", e);
//         }
//         eprintln!("Errors found, assembly failed.");
//         eprintln!();
//         std::process::exit(0);
//     }
    
//     let instructions = assembler(input.clone());
    
//     // create an OpenOptions that allows for file creation, writing, and overwriting.
//     let mut foo = OpenOptions::new().create(true).write(true).truncate(true).open(out_name.clone()).unwrap();
    
//     // not sure how to do this better.
//     for word in instructions{
//         foo.write_all(&word).unwrap();
//     }
    
//     println!("Assembly completed. Output to file: {}", out_name.clone());
// }

// fn linter(input: String) -> Vec<String>{
//     let buffer_file = File::open(input);
//     let file;
//     match buffer_file{
//         Ok(f) => {
//             file = f;
//         }
//         Err(e) => {
//             eprintln!("{}", e);
//             std::process::exit(0)
//         }
//     }
    
//     let mut errors = Vec::new();

//     let mut line_number = 1;
//     for line in BufReader::new(file).lines(){
//         let out = parse(line.unwrap());

//         if out.is_err() {
//             let e = out.err().unwrap();
//             let error = format!("{} Line number: {}", e, line_number);
//             errors.push(error);
//         }

//         line_number += 1;
//     }
    
//     return errors;
// }

// fn assembler(input: String) -> Vec<[u8; 4]>{
//     let buffer_file = File::open(input);
//     let file;
//     match buffer_file{
//         Ok(f) => {
//             file = f;
//         }
//         Err(e) => {
//             eprintln!("{}", e);
//             std::process::exit(0)
//         }
//     }
    
//     let mut instructions = Vec::new();

//     let mut line_number = 1;
//     for line in BufReader::new(file).lines(){
//         let out = parse(line.unwrap());
        
//         let p = out.ok().unwrap();

//         match p{
//             None => {}
//             Some(inst) =>{
//                 let a: [u8; 4] = inst.to_be_bytes();
//                 instructions.push(a);
//             }
//         }

//         line_number += 1;
//     }
    
//     return instructions;
// }