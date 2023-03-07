# Metainstructions

This whole file assumes that everything must be able to be run from a students implementation of RUM, with no additional structures such as an internally implemented stack.
With the current RUM instructions, there are two remaining operation codes that could be used for push and pop, which would be extremely useful and reduce overwrites.

Write down algebraic laws. AKA, break them down into their component 14 instructions.

Instructions that will require meta-programming:
- sub
- and
- or
- nor
- xor
- xnor
- not
- push
- pusha
- pop
- popa
- call
- ret

## Handling a stack
In order to handle push, pop, call, and ret, stacks will have to be implemented. The simplist way to do this will be to insert memory requests at the start of the program. These requests will thus be registered as memory location 1 and 2, and will stay there until the end of the application. Memory location 1 will be used as the function call stack. Memory location 2 will be used for pushing and popping.

### Stack Length
m[1][0] and m[2][0] will store the length of the respective stack.

### Code inserted at beginning

movi r0, 1
movi r1, #2000 ; function stack length
movi r2, 2
movi r3, #8000 ; general stack length
map r0, r1
map r2, r3
movi r0, 0
movi r1, 0
movi r2, 0
movi r3, 0

### Push
This currently requires the use of the three top registers. Unfortunatly, I'm unsure if there is any better way to do this without losing the data stored in them.

push a

movi r7, a
movi r6, #2
movi r5, #0
load r5, r6, r5
movi r6, #1
add r5, r5, r6
movi r6, #2
store r6, r5, r7

### Pop
pop a

push r0
push r1
push r2
push r3
movi r0, #2
movi r1, #0
load r1, r0, r1 ; gets the current size of the stack, which in this implementation means the top of the stack.
load r[a], r0, r1 ; puts the top of the stack into our requested register
movi r2, #1
sub r2, r1, r2
load 
sub r2, r1, r2
store r0, r0, r2


## Handling Immediates and Other Such Values
[TODO]

## Meta Operations

### Sub
sub a, b, c

push r0
push r1
movi r0, b
movi r1, c
nand r1, r1, r1
add r1, r1, #1
add r[a], r0, r1
pop r1
pop r0

### And
and a, b, c

push r0
push r1
movi r0, b
movi r1, c
nand r0, r0, r1
nand r[a], r0, r0
pop r1
pop r0

### Or (Needs optimizing)
or a, b, c

push r0
push r1
movi r0, b
movi r1, c
not r0, r0
not r1, r1
nand r[a], r0, r1
pop r0
pop r1

### Nor (Needs optimizing)
nor a, b, c

or r[a], b, c
not r[a], r[a]

### Xor (Needs optimizing)
xor a, b, c

push r0
push r1
push r2
movi r0, b
movi r1, c
or r2, r0, r1
and r0, r0, r1
not r0, r0
and r[a], r0, r2
pop r2
pop r1
pop r0

### Xnor (Needs optimizing)
xnor a, b, c

push r0
push r1
push r2
movi r0, b
movi r1, c
or r2, r0, r1
and r0, r0, r1
not r0, r0
and r0, r0, r2
nand r[a], r0, r0
pop r2
pop r1
pop r0

### Not
not a, b

push r0
movi r0, b
nand r[a], r0, r0
pop r0
