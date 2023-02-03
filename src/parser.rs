use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref OPS: HashMap<&'static str, (u32, u32)> = {
        let mut m = HashMap::new();
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

#[macro_export]
macro_rules! max_num {
    () => {
        2_u32.pow(25)
    };
}

/// Returns an instruction based on the given string.
pub fn parse(string: String) -> Result<Option<u32>, String> {
    let a = string.trim();
    if string == "" || string.contains(":"){
        return Ok(None);
    }

    // split the string into a buffer
    let a = a.split_whitespace();
    // get the count of the separated words
    let arg_count = a.clone().count();
    // collect the args into a vector
    let args: Vec<&str> = a.collect();

    let op = args[0];

    // Is the operation found?
    if !OPS.contains_key(op){
        return Err(format!("[ERROR] Invalid operation: \"{}\".", op));
    }

    let (opcode, argc) = *OPS.get(op).unwrap();

    // Check if argc matches expected (This is the same as argc in C, wherein the op is included.)
    if arg_count != argc as usize {
        return Err(format!("[ERROR] \"{}\" expects {} arguments!", op, argc - 1));
    }

    match opcode{
        7 => { Ok(Some(halt())) }
        13 => { parse_movi(opcode, args)}
        _ => {
            if argc == 2 { parse_2_arg(opcode, args)}
            else if argc == 3 { parse_3_arg(opcode, args)}
            else if argc == 4 { parse_4_arg(opcode, args)}
            else{
                Err(format!("[ERROR] Assembler broke REAL BAD. This should never happen as it requires a contradiction"))
            }
        }
    }
}

fn parse_4_arg(op: u32, args: Vec<&str>) -> Result<Option<u32>, String>{
    let n1 = args[1].find("r");
    let n2 = args[2].find("r");
    let n3 = args[3].find("r");

    if n1 == None || n2 == None || n3 == None {
        return Err(format!("[ERROR] Register(s) not preceded with 'r'!"));
    }

    let (_, ra) = args[1].split_at(n1.unwrap() + 1);
    let (_, rb) = args[2].split_at(n2.unwrap() + 1);
    let (_, rc) = args[3].split_at(n3.unwrap() + 1);
    
    let ra = ra.replace(",", "");
    let rb = rb.replace(",", "");
    let rc = rc.replace(",", "");
    
    let reg_a = ra.parse::<u32>().unwrap();
    let reg_b = rb.parse::<u32>().unwrap();
    let reg_c = rc.parse::<u32>().unwrap();
    
    let instruction = op << 28 | (reg_a << 6) | (reg_b << 3) | reg_c;
    Ok(Some(instruction))
}

fn parse_3_arg(op: u32, args: Vec<&str>) -> Result<Option<u32>, String>{
    let n1 = args[1].find("r");
    let n2 = args[2].find("r");

    if n1 == None || n2 == None {
        return Err(format!("[ERROR] Register(s) not preceded with 'r'!"));
    }

    let (_, rb) = args[1].split_at(n1.unwrap() + 1);
    let (_, rc) = args[2].split_at(n2.unwrap() + 1);

    let rb = rb.replace(",", "");
    let rc = rc.replace(",", "");

    let reg_b = rb.parse::<u32>().unwrap();
    let reg_c = rc.parse::<u32>().unwrap();

    let instruction = op << 28 | (reg_b << 3) | reg_c;
    Ok(Some(instruction))
}

fn parse_2_arg(op: u32, args: Vec<&str>) -> Result<Option<u32>, String>{
    let n1 = args[1].find("r");

    if n1 == None {
        return Err(format!("[ERROR] Register(s) not preceded with 'r'!"));
    }

    let (_, rc) = args[1].split_at(n1.unwrap() + 1);

    let rc = rc.replace(",", "");

    let reg_c = rc.parse::<u32>().unwrap();

    let instruction = op << 28 | reg_c;
    Ok(Some(instruction))
}

fn parse_movi(op: u32, args: Vec<&str>) -> Result<Option<u32>, String>{
    let n1 = args[1].find("r");

    if n1 == None {
        return Err(format!("[ERROR] Register not preceded with 'r'!"));
    }

    let (_, rl) = args[1].split_at(n1.unwrap() + 1);
    let (_, temp) = args[2].split_at(1);
    let lv = temp.parse::<u32>();
    
    match lv{
        Err(_) => {
            return Err(format!("[ERROR] Could not parse immediate value! Did you forget the '#' before the value?"));
        }
        Ok(_) => {}
    }
    
    let lv = lv.unwrap();
    
    if lv >= max_num!(){
        // just in case
        return Err(format!("[ERROR] Immediate value exceeded 25 bits! value: {}.", lv));
    }

    let rl = rl.replace(",", "");
    //let reg = *REGS.get(&*rl).unwrap();
    let reg = rl.parse::<u32>().unwrap();
    let instruction = op << 28 | (reg << 25) | lv;
    Ok(Some(instruction))
}

fn halt() -> u32{
    7 << 28
}