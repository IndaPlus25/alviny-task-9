


use std::{collections::HashMap, env::args, fmt::Error, io};

#[derive(Clone, PartialEq, Eq, Hash)]
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
#[derive(Clone)]
struct CPU {
    registers: HashMap<Register, u16>
}
impl CPU {
    fn new() -> CPU {
        CPU {registers: HashMap::from([
            (Register::Zero, 0),
            (Register::Stdio, 0),
            (Register::Iter0, 0),
            (Register::Iter1, 0),
            (Register::Cond0, 0),
            (Register::Cond1, 0),
            (Register::Temp0, 0),
            (Register::Temp1, 0),
            (Register::Temp2, 0),
            (Register::Arg0, 0),
            (Register::Arg1, 0),
            (Register::Arg2, 0),
            (Register::Save0, 0),
            (Register::Save1, 0),
            (Register::Save2, 0),
            (Register::Save3, 0),

        ])}
    }
    fn execute_operation(&mut self, op: String) {
        todo!()
    } 
}

fn parse_file(file: &String) -> Result<Vec<String>, Error> {
    // failsafe: check if file ending is correct. If not, error
    
    todo!();
}
fn main() {
    let args: Vec<String> = args().collect();
    let mut cpu = CPU::new();

    let statements = parse_file(&args[1]).ok().expect("Error: Could not parse Alnum file!");
    for i in statements {
        cpu.execute_operation(i);
    }
    println!("---Program finished running (Exit code: 0)---");
}
