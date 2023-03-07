use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while};
use nom::character::complete::{alpha0, alphanumeric0, char, digit0};
use nom::sequence::{tuple, terminated, preceded};
use nom::IResult;
use nom::multi::many0;

// fn parse_op(input: &str) -> IResult<&str, u32>{
//     alt((
//         tag("cmov"),
//         tag("load"),
//         tag("store"),
//         tag("add")
//     ))(input)
// }

fn delimiter(input: &str) -> IResult<&str, &str> {
    tag(", ")(input)
}

fn separator(input: &str) -> IResult<&str, &str>{
    tag(" ")(input)
}

fn reg_exp(input: &str) -> IResult<&str, &str> {
    preceded(tag("r"), digit0)(input)
}

fn num_exp(input: &str) -> IResult<&str, &str> {
    digit0(input)
}

fn hex_exp(input: &str) -> IResult<&str, &str> {
    preceded(tag("0x"),alphanumeric0)(input)
}

fn is_binary(c: char) -> bool{
    c == '0' || c == '1'
}

fn bin_exp(input: &str) -> IResult<&str, &str> {
    preceded(tag("0b"),take_while(is_binary))(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    alt((bin_exp, hex_exp, num_exp))(input)
}

fn param_exp(input: &str) -> IResult<&str, &str> {
    
    alt((
        reg_exp,
        preceded(tag("#"), value)
        
    ))(input)
}

fn p0_op(input: &str) -> IResult<&str, &str>{
    alt((
        tag_no_case("halt"),
        tag_no_case("ret")
    ))(input)
}

fn p_movi(input: &str) -> IResult<&str, &str>{
    tag("movi")(input)
}

fn p1_op(input: &str) -> IResult<&str, &str>{
    alt((
        tag_no_case("umap"),
        tag_no_case("out"),
        tag_no_case("in"),
        tag_no_case("push"),
        tag_no_case("pop"),
        tag_no_case("call"),
    ))(input)
}

fn p2_op(input: &str) -> IResult<&str, &str>{
    alt((
        tag_no_case("not"),
        tag_no_case("map"),
        tag_no_case("lp")
    ))(input)
}

fn p3_op(input: &str) -> IResult<&str, &str>{
    alt((
        tag_no_case("cmov"),
        tag_no_case("load"),
        tag_no_case("store"),
        tag_no_case("add"),
        tag_no_case("sub"),
        tag_no_case("mul"),
        tag_no_case("div"),
        tag_no_case("and"),
        tag_no_case("nand"),
        tag_no_case("or"),
        tag_no_case("nor"),
        tag_no_case("xor"),
        tag_no_case("xnor")
    ))(input)
}

fn operation(input: &str) -> IResult<&str, u32>{
    
    let out = alt((
        p0_op,
        p1_op,
        p2_op,
        p3_op
    ))(input);
    // match out{
    //     Ok((rem, out)) => {
    //         
    //     }
    //     Err(_) => {}
    // }

    Ok(("", 0))
}

pub fn parse_line(input: &str) -> IResult<&str, u32>{
    
    
    
    let (rem, op) = terminated(operation, separator)(input)?;
    
    let (rem, p1) = terminated(param_exp, delimiter)(rem)?;

    println!("'{}'", op);
    println!("'{}'", p1);
    println!("{}", rem);
    
    Ok(("", 0))
}

pub fn parse(filename: &str){
    let buffer = match File::open(filename){
        Ok(file) => file,
        Err(e) => panic!("Could not read file!")
    };
    
    for line in BufReader::new(buffer).lines(){
        if line.is_err(){
            panic!("Could not read line!");
        }
        
    }
    
}

fn get_opcode(input: &str) -> Result<u32, &str>{
    match input{
        "cmov" => Ok(0),
        "load" => Ok(1),
        "store" => Ok(2),
        "add" => Ok(3),
        "mul" => Ok(4),
        "div" => Ok(5),
        "nand" => Ok(6),
        "halt" => Ok(7),
        "map" => Ok(8),
        "umap" => Ok(9),
        "out" => Ok(10),
        "in" => Ok(11),
        "lp" => Ok(12),
        "movi" => Ok(13),
        "sub" => Ok(14),
        "and" => Ok(15),
        "or" => Ok(16),
        "nor" => Ok(17),
        "xor" => Ok(18),
        "xnor" => Ok(19),
        "not" => Ok(20),
        "push" => Ok(21),
        "pop" => Ok(22),
        "call" => Ok(23),
        "ret" => Ok(24),
        n => Err("Unidentified operation!")
    }
}