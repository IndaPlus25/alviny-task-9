# az09 Language specification

The language name is pronounced "Ay Zed Zero Nine" and is based on the regex that matches the character set that the language uses `[a-z0-9#\- ]`.

The language uses 16-bit instructions,  and 4 bit register addresses (16 registers).

The language has 8 operations.

Each register is 16 bits in size and is treated as unsigned by default.

## Instructions

| **Type**  | **Encoding** |
|:----------|:-------------|
| Register  | `op<15:13>, rs<12:9>, rt<8:5>, rd<4:1>` |
| Immediate, 2 registers | `op<15:13>, rs<12:9>, rt<8:5>, imm<4:0>`|
| Immediate, 1 register  | `op<15:13>, rs<12:9>, imm<8:0>`|
| Immediate, no register | `op<15:13>, imm<12:0>` |

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

syscall imm<9 bits, unsigned> reg1: Calls system function number imm using register reg1, similar to RISC-V's ecall.
```

### Immediate Instructions, no register

```alnum
jump imm<13 bits, signed>: jumps imm steps.
```

### Syscall numbers

0: ReadUInt. Reads an integer from stdin and writes to reg1.

1: PrintUInt. Reads an unsigned integer from reg1 and puts it in stdout.

2: PrintInt. Reads a signed integer from reg1 and puts it in stdout.

3: PrintChar. Reads an integer from reg1 and puts it in stdout as an ASCII char. Errors if the stdio register is larger than the maximum ASCII char index.

4: Exit program with unsigned integer code specified in reg1.

5: Exit program with signed integer code specified in reg1.

### Interpreter instructions

hash symbol (#): everything after this until the next newline is a comment.

## Registers

The Alnum language has 16 named registers, split into 4 sectors.

```code
Binary - Name  - Description: Recommended usage (if any)

Sector 00: Special registers ------------------------------
Binary - Name  - Description: Recommended usage (if any)
0000   - zero  - always equal to 0.
0001   - iter0 - Temporary variable 0: iterator variable 1
0010   - iter1 - Temporary variable 1: iterator variable 2
0011   - cond  - Temporary variable 2: conditional variable

Sector 01: Temporary variables ----------------------------
Binary - Name  - Description: Recommended usage (if any)
0100   - temp0 - Temporary variable 3
0101   - temp1 - Temporary variable 4
0110   - temp2 - Temporary variable 5
0111   - temp3 - Temporary variable 6

Sector 10: Arguments --------------------------------------
1000   - arg0  - Argument 0: Syscall buffer
1001   - arg1  - Argument 1: Return Buffer
1010   - arg2  - Argument 2
1011   - arg3  - Argument 3

Sector 11: Saved Variables --------------------------------
Binary - Name  - Description: Recommended usage (if any)
1100   - save0 - Saved variable 0
1101   - save1 - Saved variable 1
1110   - save2 - Saved variable 2
1111   - save3 - Saved variable 3
```
