use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Deref;
use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_till, take_until, take_while};
use nom::character::complete::{alpha0, alphanumeric0, char, digit0, one_of};
use nom::combinator::recognize;
use nom::error::{context, VerboseError};
use nom::sequence::{tuple, terminated, preceded};
use nom::IResult;
use nom::multi::{many0, many1, many_till};
use crate::linter_errors::FormattingError;

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

macro_rules! op {
    ($operation:expr) => {
        terminated(tag_no_case($operation), separator)
    }
}

// fn param_exp(input: &str) -> IResult<&str, &str> {
//     
//     alt((
//         reg_exp,
//         preceded(tag("#"), value)
//         
//     ))(input)
// }

pub fn parse_line(input: &str) -> u32{
    //let out = terminated(operation, separator)(input);

    let buffer = operation(input);
    
    let out = buffer.unwrap();
    out.1
}

pub fn parse(filename: &str) -> Vec<[u8; 4]>{
    let buffer = match File::open(filename){
        Ok(file) => file,
        Err(e) => panic!("Could not read file!")
    };
    
    let mut output = Vec::new();
    
    for line in BufReader::new(buffer).lines(){
        if line.is_err(){
            panic!("Could not read line!");
        }

        let out = parse_line(line.unwrap().as_str());
        output.push(out.to_be_bytes());
    }
    
    output
}

fn file_name(input: &str) -> IResult<&str, &str>{
    take_until(".")(input)
}

fn operation(input: &str) -> IResult<&str, u32>{

    alt((
        p0_op,
        p1_op,
        p2_op,
        p3_op,
        p_movi
    ))(input)
    
}

fn p0_op(input: &str) -> IResult<&str, u32>{
    let (rem, op) = halt(input)?;
    Ok(("", OPS.get(op).unwrap().clone().0 << 28))
}

fn p_movi(input: &str) -> IResult<&str, u32>{
    let (rem, op) = terminated(tag_no_case("movi"), separator)(input)?;
    let (rem, lr) = terminated(reg_exp, delimiter)(rem)?;
    let (rem, lv) = value(rem)?;
    
    let reg_l = lr.parse::<u32>().unwrap();;
    
    let inst = (OPS.get(op).unwrap().clone().0 << 28) | (reg_l << 25) | lv;

    Ok(("", inst))
}

fn p1_op(input: &str) -> IResult<&str, u32>{
    let (rem, op) = context("Test context", alt((
        op!("umap"),
        op!("out"),
        op!("in"),
    )))(input)?;
    
    let (rem, rc) = reg_exp(rem)?;

    let reg_c = rc.parse::<u32>().unwrap();

    let inst = (OPS.get(op).unwrap().clone().0 << 28) | reg_c;

    Ok(("", inst))
}

fn p2_op(input: &str) -> IResult<&str, u32>{
    let (rem, op) = alt((
        op!("map"),
        op!("lp")
    ))(input)?;

    let (rem, rb) = terminated(reg_exp, delimiter)(rem)?;
    let (rem, rc) = reg_exp(rem)?;

    let reg_b = rb.parse::<u32>().unwrap();
    let reg_c = rc.parse::<u32>().unwrap();

    let inst = (OPS.get(op).unwrap().clone().0 << 28) | (reg_b << 3) | reg_c;

    Ok(("", inst))
}

fn p3_op(input: &str) -> IResult<&str, u32>{
    let (rem, op) = alt((
        op!("cmov"),
        op!("load"),
        op!("store"),
        op!("add"),
        op!("mul"),
        op!("div"),
        op!("nand"),
    ))(input)?;

    let (rem, ra) = terminated(reg_exp, delimiter)(rem)?;
    let (rem, rb) = terminated(reg_exp, delimiter)(rem)?;
    let (rem, rc) = reg_exp(rem)?;

    let reg_a = ra.parse::<u32>().unwrap();
    let reg_b = rb.parse::<u32>().unwrap();
    let reg_c = rc.parse::<u32>().unwrap();

    let inst = (OPS.get(op).unwrap().clone().0 << 28) | (reg_a << 6) | (reg_b << 3) | reg_c;
    
    Ok(("", inst))
}

fn halt(input: &str) -> IResult<&str, &str>{
    tag_no_case("halt")(input)
}

fn delimiter(input: &str) -> IResult<&str, &str> {
    tag(", ")(input)
}

fn separator(input: &str) -> IResult<&str, &str>{
    tag(" ")(input)
}

fn reg_exp(input: &str) -> IResult<&str, &str> {
    preceded(tag("r"), digit0)(input)
}

fn num_exp(input: &str) -> IResult<&str, u32> {
    let (rem, v) = preceded(tag("#"), digit0)(input)?;
    Ok((rem, v.parse::<u32>().unwrap()))
}

fn hex_exp(input: &str) -> IResult<&str, u32> {
    let (rem, v) = preceded(alt((tag("0x"), tag("0X"))),
                            recognize(
                                many1(
                                    terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
                                )
                            )
    )(input)?;
    let v = u32::from_str_radix(v, 16).unwrap();
    Ok((rem, v))
}

fn is_binary(c: char) -> bool{
    c == '0' || c == '1'
}

fn bin_exp(input: &str) -> IResult<&str, u32> {
    let (rem, v) = preceded(tag("0b"),take_while(is_binary))(input)?;
    let v = u32::from_str_radix(v, 2).unwrap();
    Ok((rem, v))
}

fn value(input: &str) -> IResult<&str, u32> {
    alt((bin_exp, hex_exp, num_exp))(input)
}