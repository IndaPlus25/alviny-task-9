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

syscall imm<9 bits, unsigned> reg1: Calls system function number imm using register reg1, similar to RISC-V's ecall.
```

### Syscall numbers

0: ReadInt. Reads an integer from stdin and writes to reg1.

1: PrintUInt. Reads an unsigned integer from reg1 and puts it in stdout.

2: PrintInt. Reads a signed integer from reg1 and puts it in stdout.

3: PrintChar. Reads an integer from reg1 and puts it in stdout as an ASCII char. Errors if the stdio register is larger than the maximum ASCII char index.

4: Exit program with unsigned integer code specified in reg1.

5: Exit program with signed integer code specified in reg1.

### Interpreter instructions

hash symbol (#): everything after this until the next newline is a comment.

## Registers

The Alnum language has 16 named registers.

```code
Binary - Name  - Description: Recommended usage (if any)
--------------------------------------------------------------
0000   - zero  - always equal to 0.
0001   - stdio - standard IO buffer.
0010   - iter0 - Temporary variable 1: iterator variable 1
0011   - iter1 - Temporary variable 2: iterator variable 2
0100   - cond0 - Temporary variable 3: conditional variable 1
0101   - cond1 - Temporary variable 4: conditional variable 2
0110   - temp0 - Temporary variable 5
0111   - temp1 - Temporary variable 6
1000   - temp2 - Temporary variable 7
1001   - arg0  - Argument 1: Syscall argument
1010   - arg1  - Argument 2
1011   - arg2  - Argument 3
1100   - save0 - Saved variable 1
1101   - save1 - Saved variable 2
1110   - save2 - Saved variable 3
1111   - save3 - Saved variable 4
```
