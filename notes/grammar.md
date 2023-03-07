Much of this similar due to research of C grammar

NOTE: Curly brackets {} indicate a list of possible outcomes.
Take it to be that each line can be concatenated into a
single line separated by an OR operator (|).



<letters> ::= "a" | ... | "z" | "A" | ... | "Z"

<base10-val> ::= "0" | ... | "9"
<hex-val> ::= <base10-val> | "a" | ... | "f" | "A" | ... | "F"
<bin-val> ::= "0" | "1"

<num-exp> :: = <base10-val> | <num-exp> <base10-val>
<hex-exp> ::= "0x" <hex-val> | <hex> <hex-val>
<bin-exp> ::= "0b" <bin-val> | <bin> <bin-val>

<value> = <num-exp> | <hex-exp> | <bin-exp>

<program> ::= <program> <inst> | <program> <inst> <EOF>
<inst> ::= <op-exp> <EOL>
<op-exp> ::= {
    "cmov" <param-exp> "," <param-exp> "," <param-exp>
    "load" <param-exp> "," <param-exp> "," <param-exp>
    "store" <param-exp> "," <param-exp> "," <param-exp>
    "add" <param-exp> "," <param-exp> "," <param-exp>
    "sub" <param-exp> "," <param-exp> "," <param-exp>
    "mul" <param-exp> "," <param-exp> "," <param-exp>
    "div" <param-exp> "," <param-exp> "," <param-exp>
    "and" <param-exp> "," <param-exp> "," <param-exp>
    "nand" <param-exp> "," <param-exp> "," <param-exp>
    "or" <param-exp> "," <param-exp> "," <param-exp>
    "nor" <param-exp> "," <param-exp> "," <param-exp>
    "xor" <param-exp> "," <param-exp> "," <param-exp>
    "xnor" <param-exp> "," <param-exp> "," <param-exp>
    "not" <param-exp> "," <param-exp>
    "halt
    "map" <param-exp> "," <param-exp>
    "umap" <param-exp>
    "out" <param-exp>
    "in" <param-exp>
    "lp" <param-exp> "," <param-exp>
    "movi" <param-exp> "," <immediate-exp>
    "push" <param-exp>
    "pop" <param-exp>
    "call" <param-exp>
    "ret"
}

<param-exp> ::= <reg-exp> | "#" <value> | "#(" <mul-exp> ")" | "[" <mul-exp> "]"
<reg-exp> ::= { ; This grammar doesn't care about how high the num-exp goes, just that it's a valid num-exp.
    "r" <num-exp>
    "r[" <value> "]"
}
<mul-exp> ::= {
    <mul-exp> * <mul-exp>
    "(" <mul-exp> ")"
    "(" <add-sub-exp> ")"
    <value>
}

<add-sub-exp> ::={
    <mul-exp> "+" <mul-exp>
    <mul-exp> "-" <mul-exp>
}