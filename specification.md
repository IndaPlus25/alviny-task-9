# Alnum Language specification


The Alnum language uses 16-bit instructions, 16 bit register size, and 3 bit register numbers.

## Instructions

| **Type** | **Encoding** |
|:---------|:-------------|
| Register | `op<15:14>, rs<13:11>, rt<10:8>, rd<7:5>, imm<4:0>` |
| Immediate | `op<15:14>, rs<13:11>, rt<10:8>, imm<7:0>` |
| Jump     | `op<7:5>, addr<4:0>` |
| Special  | `op<7:5>` |

### Register instructions


## Registers

The Alnum language has 8 registers
```python
000 - zero  - always equals 0.
001 - stdio - standard IO stream.
010 - Arg1  - Temporary variable 1
011 - Arg2  - Temporary variable 2
100 - iter  - Temporary variable 3: iterator variable
101 - cond  - temporary variable 4: Conditional
110 - Save1 - Saved Variable 1
111 - Save2 - Saved Variable 2
```