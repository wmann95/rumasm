use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use lazy_static::lazy_static;
use rumasm::error::Error;

lazy_static! {
    static ref OPS: std::collections::HashMap<&'static str, (u32, u32)> = {
        let mut m = std::collections::HashMap::new();
        m.insert("cmov", (0, 4));
        m.insert("load", (1, 4));
        m.insert("store", (2, 4));
        m.insert("add", (3, 4));
        m.insert("mul", (4, 4));
        m.insert("div", (5, 4));
        m.insert("nand", (6, 4));
        m.insert("halt", (7, 1));
        m.insert("map", (8, 3));
        m.insert("umap", (9, 2));
        m.insert("out", (10, 2));
        m.insert("in", (11, 2));
        m.insert("lp", (12, 3));
        m.insert("movi", (13, 3));
        m
    };
}
lazy_static! {
    static ref REGS: std::collections::HashMap<&'static str, u32> = {
        let mut m = std::collections::HashMap::new();
        m.insert("r0", 0);
        m.insert("r1", 1);
        m.insert("r2", 2);
        m.insert("r3", 3);
        m.insert("r4", 4);
        m.insert("r5", 5);
        m.insert("r6", 6);
        m.insert("r7", 7);
        m
    };
}

#[macro_export]
macro_rules! MAX_NUM {
    () => {
        2_u32.pow(25);
    };
}

fn main(){
    let input = args().nth(1);
    let output = args().nth(2);
    
    if input == None {
        println!("Usage:");
        println!("   rumasm [INPUT_FILE]");
        println!("   rumasm [INPUT_FILE] [OUTPUT_FILE]");
        std::process::exit(0)
    }
    
    let bfile = File::open(input.unwrap());
    let mut file;
    match bfile{
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0)
        }
    }
    
    let mut out_name = match output{
        Some(s) => {s},
        None => ("out.bin".to_string())
    };
    
    let mut foo = OpenOptions::new().create(true).write(true).truncate(true).open(out_name).unwrap();
    let mut line_number = 1;
    for line in BufReader::new(file).lines(){
        let out = parse(line.unwrap());
        
        if out.is_err() {
            let e = out.err().unwrap();
            eprintln!("{}. Line number: {}", e, line_number);
            panic!();
        }
        
        let p = out.ok().unwrap();
        
        match p{
            None => {}
            Some(inst) =>{
                let a: [u8; 4] = inst.to_be_bytes();
                foo.write_all(&a).unwrap();
            }
        }
        
        line_number += 1;
    }
}

fn parse(string: String) -> Result<Option<u32>, String> {
    if string == ""{
        return Ok(None);
    }
    
    // split the string into a buffer
    let mut a = string.split_whitespace();
    // get the count of the separated words
    let arg_count = a.clone().count();
    // collect the args into a vector
    let mut args: Vec<&str> = a.collect();
    
    let op = args[0];
    
    if !OPS.contains_key(op){
        return Err(format!("[ERROR] Invalid operation: {}", op));
    }
    
    let (opcode, argc) = *OPS.get(op).unwrap();
    
    if arg_count != argc as usize {
        return Err(format!("[ERROR] Expected {} arguments!", argc));
    }
    
    let mut instruction = opcode;
    instruction = instruction << 28;

    if opcode == 13 {
        let (_, rl) = args[1].split_at(1);
        let (_, temp) = args[2].split_at(1);
        let lv = temp.parse::<u32>().unwrap();
        let rl = rl.replace(",", "");
        let reg = *REGS.get(&*rl).unwrap();
        instruction = instruction | (reg << 25) | lv;
    }
    else if arg_count == 2{
        let (_, rc) = args[1].split_at(1);
        let rc = rc.replace(",", "");
        let regc = *REGS.get(&*rc).unwrap();
        instruction = instruction | regc;
        
    }
    else if arg_count == 3{
        let (_, rb) = args[1].split_at(1);
        let (_, rc) = args[2].split_at(1);
        let rb = rb.replace(",", "");
        let rc = rc.replace(",", "");
        let regb = *REGS.get(&*rb).unwrap();
        let regc = *REGS.get(&*rc).unwrap();
        instruction = instruction | (regb << 4) | regc;
    }
    else if arg_count == 4{
        let (_, ra) = args[1].split_at(1);
        let (_, rb) = args[2].split_at(1);
        let (_, rc) = args[3].split_at(1);
        let ra = ra.replace(",", "");
        let rb = rb.replace(",", "");
        let rc = rc.replace(",", "");
        let rega = *REGS.get(&*ra).unwrap();
        let regb = *REGS.get(&*rb).unwrap();
        let regc = *REGS.get(&*rc).unwrap();
        instruction = instruction | (rega << 8) | (regb << 4) | regc;
    }
    
    return Ok(Some(instruction));
}