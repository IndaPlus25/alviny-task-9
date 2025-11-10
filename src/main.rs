/* syntax:
statements:
assign register to op|imm: assigns a reg to an operation
operations:
    register1 plus register2: adds the value of register1 plus register2 and returns the sum.
    register1 times register2: multiplies the value of register1 and register2 and returns the product.
    
*/


use std::{env::args, io};

enum Register{
    Zero,
    Stdio,
    Iter0,
    Iter1,
    Cond0,
    Cond1,
    Temp0,
    Temp1,
    Temp2,
    Arg0,
    Arg1,
    Arg2,
    Save0,
    Save1,
    Save2,
    Save3,
}

fn register_index(register: Register) -> u8 {
    match register{
        Register::Zero => 0,
        Register::Stdio=> 1,
        Register::Iter0=> 2,
        Register::Iter1=> 3,
        Register::Cond0=> 4,
        Register::Cond1=> 5,
        Register::Temp0=> 6,
        Register::Temp1=> 7,
        Register::Temp2=> 8,
        Register::Arg0 => 9,
        Register::Arg1 => 10,
        Register::Arg2 => 11,
        Register::Save0=> 12,
        Register::Save1=> 13,
        Register::Save2=> 14,
        Register::Save3=> 15,
    }
}

fn parse_file(file: &String) -> Vec<String> {
    // first failsafe: check if file in 
    todo!();
}

fn execute_operation(op: String, regs: Vec<u16>) {
    todo!();
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut registers: Vec<u16> = vec![0; 16];

    // this is pseudocode
    let statements = parse_file(&args[1]);
    for i in statements {
        execute_operation(i);
    }
    println!("---Program finished running (Exit code: 0)---");
}
