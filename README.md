# rumasm
RUM Assembler

## Description
A simple assembler for RUM, the Rust-implementation Universal Machine. This does not check validity of code, meaning that it does not check if the registers used are valid or if the immediate values entered will fit within the available bits.

## Usage
Create a file that follows my formatting below. I chose to use the filetype ".rumasm" but this is not necessary.
Use "cargo build -r" to build the program
Use "./target/release/rumasm". This will show you the usage in your terminal, but use as follows: "rumasm [INPUT_FILE_PATH] [OUTPUT_FILE_PATH]"
If there are any errors, they will be listed in your terminal.
If all succeeds, the assembly will be output to either OUTPUT_FILE_PATH or "out.bin" if none was given.
Run the output through RUM.

## Formatting

### 3 Argument Ops
op ra, rb, rc

Example: "cmov r1, r4, r2"

### 2 Argument Ops
op rb, rc

Example: "map r2, r3"

### 1 Argument Ops
op rc

Example "out r0"

### 0 Argument Op
halt

### Load Value
movi ra, #value

Example: "movi r1, #100"
