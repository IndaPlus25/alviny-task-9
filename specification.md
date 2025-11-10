# Alnum Language specification

The language uses 16-bit instructions,  and 4 bit register addresses (16 registers).

The language has 8 operations.

Each register is 16 bits in size and is treated as unsigned by default.

## Instructions

| **Type**  | **Encoding** |
|:----------|:-------------|
| Register  | `op<15:13>, rs<12:9>, rt<8:5>, rd<4:1>` |
| Immediate, 2 registers | `op<15:13>, rs<12:9>, rt<8:5>, imm<4:0>`|
| Immediate, 1 register  | `op<15:13>, rs<12:9>, imm<8:0>`|

### Register instructions

```alnum
assign reg1 to reg2 plus reg3:  reg1 = reg2 + reg3
assign reg1 to reg2 minus reg3: reg1 = reg2 - reg3
```

### Immediate Instructions, 2 registers

```alnum
immassign reg1 to reg2 plus imm<5 bits, signed>: reg1 = reg2 + imm

jump imm<5 bits, signed> if reg1 equals reg2: Jump imm steps if reg1 == reg2
jump imm<5 bits, signed> if reg1 greaterthan reg2: Jump imm steps if reg1 > reg2
```

### Immediate Instructions, 1 register

```alnum
immassign reg1 to imm<9 bits, unsigned>: reg1 = imm

bitshift reg1 by imm<9 bits, signed>: reg1 << imm if imm > 0, else reg 1 >> imm.
syscall imm<9 bits, unsigned> reg1: Calls system function number imm and writes to reg1.
```

### Syscall numbers

0: ReadInt. Reads an integer from stdin and puts it in the stdio register.

1: PrintUInt. Reads an unsigned integer from the stdio register and puts it in stdout.

2: PrintInt. Reads a signed integer from the stdio register and puts it in stdout.

2: PrintChar. Reads an integer from the stdio register and puts it in stdout as an ASCII char. Errors if the stdio register is larger than the maximum ASCII char index.

3: Exit program with code 0.

4: Exit program with code specified in the stdio register.

### Interpreter instructions

hash symbol (#): everything after this until the next newline is a comment.

## Registers

The Alnum language has 16 registers.

```code
0000 - zero  - always equals 0.
0001 - stdio - standard IO buffer.
0010 - iter0 - Temporary variable 1: iterator variable 1
0011 - iter1 - Temporary variable 2: iterator variable 2
0100 - cond0 - Temporary variable 3: conditional variable 1
0101 - cond1 - temporary variable 4: conditional variable 2
0110 - temp0 - temporary variable 5
0111 - temp1 - temporary variable 6
1000 - temp2 - temporary variable 7
1001 - arg0  - argument 1: syscall argument
1010 - arg1  - argument 2
1011 - arg2  - argument 3
1100 - save0 - saved variable 1
1101 - save1 - saved variable 2
1110 - save2 - saved variable 3
1111 - save3 - saved variable 4
```
