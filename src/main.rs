


use std::{collections::HashMap, env::args, fs, io::{self}, process::exit};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Register{
    Zero,
    Iter0,
    Iter1,
    Cond,

    Temp0,
    Temp1,
    Temp2,
    Temp3,

    Arg0,
    Arg1,
    Arg2,
    Arg3,

    Save0,
    Save1,
    Save2,
    Save3,
}
#[derive(Debug)]
enum InterpreterError{
    NotAlnumError,
    SyntaxError,
    BadRegisterError,
    BadImmediateError

}

//Emulate CPU
#[derive(Clone, Debug)]
struct CPU {
    registers: HashMap<Register, u16>
}
impl CPU {
    fn new() -> CPU {
        CPU {registers: HashMap::from([
            (Register::Zero, 0),
            (Register::Iter0, 0),
            (Register::Iter1, 0),
            (Register::Cond, 0),
            (Register::Temp0, 0),
            (Register::Temp1, 0),
            (Register::Temp2, 0),
            (Register::Temp3, 0),
            (Register::Arg0, 0),
            (Register::Arg1, 0),
            (Register::Arg2, 0),
            (Register::Arg3, 0),
            (Register::Save0, 0),
            (Register::Save1, 0),
            (Register::Save2, 0),
            (Register::Save3, 0),

        ])}
    }

    // register instructions
    fn assign_plus(&mut self, reg1: Register, reg2: Register, reg3: Register) -> usize {
        todo!()
    }
    fn assign_minus(&mut self, reg1: Register, reg2: Register, reg3: Register) -> usize {
        todo!()
    }

    //immediate instructions, 2 registers
    fn immassign_plus(&mut self, reg1: Register, reg2: Register, imm: i8) -> usize {
        todo!()
    }
    fn jump_if_eq(&mut self, reg1: Register, reg2: Register, imm: i8) -> usize {
        todo!()
    }
    fn jump_if_greater(&mut self, reg1: Register, reg2: Register, imm: i8) -> usize {
        todo!()
    }

    // Immediate instructions, 1 register
    fn immassign_imm(&mut self, reg1: Register, imm: u16) -> usize {
        todo!()
    }
    fn syscall(&mut self, reg1: Register, imm: u16) -> usize {
        todo!()
    }

    //Immediate instructions, no register

    fn jump(imm: i16) -> usize {
        todo!()
    }

    
}

//System instructions here

//Function that returns a Register from a register name in string form

fn get_register(regname: &str) -> Result<Register, InterpreterError> {
    return match regname {
        //sector 00
        "zero" => Ok(Register::Zero),
        "iter0" => Ok(Register::Iter0),
        "iter1" => Ok(Register::Iter1),
        "cond" => Ok(Register::Cond),

        //sector 01
        "temp0" => Ok(Register::Temp0),
        "temp1" => Ok(Register::Temp1),
        "temp2" => Ok(Register::Temp2),
        "temp3" => Ok(Register::Temp3),

        // sector 10
        "arg0" => Ok(Register::Arg0),
        "arg1" => Ok(Register::Arg1),
        "arg2" => Ok(Register::Arg2),
        "arg3" => Ok(Register::Arg3),

        // sector 11
        "save0" => Ok(Register::Save0),
        "save1" => Ok(Register::Save1),
        "save2" => Ok(Register::Save2),
        "save3" => Ok(Register::Save3),

        // error on invalid register
        _ => Err(InterpreterError::BadRegisterError)
    }
}


fn parse_file(file_path: &String) -> Result<Vec<String>, InterpreterError> {
    // failsafe: check if file ending is correct. If not, error
    let mut operations = match *(file_path.split('.').collect::<Vec<_>>().last().unwrap()) == "anm" {
        true => fs::read_to_string(file_path).expect("Unable to read input file"),
        false => return Err(InterpreterError::NotAlnumError),
    };

    // split by newline
    let mut operations_split:Vec<String> = operations.split("\n")
    .collect::<Vec<&str>>()
    .into_iter()
    .map(|x| 
        x.strip_suffix("\r")
        .unwrap_or(x)
        // we remove comments here, in the same map function
        .split('#')
        .collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
        
    )
    .collect::<Vec<String>>();
    // purge blank lines
    operations_split.retain(|x| !x.is_empty());
    println!("{:?}", operations_split);

    // we now have a vector full of statements. 
    // check each statement for disallowd characters (Capital letters, commas, period, )
    
    //return it
    Ok(operations_split)
}
fn main() {
    let args: Vec<String> = args().collect();
    let mut cpu = CPU::new();

    let statements_result = parse_file(&args[1]);
    // An error here means you haven't entered a file with the correct file ending.
    let statements = match statements_result {
        Ok(vec) => vec,
        Err(error) => panic!("Interpreter Error: {error:?}"),
    };
    println!("{:?}", statements);
    
    let mut line_counter: usize= 1;
    while line_counter <= statements.len().try_into().unwrap() {
        let op = statements[line_counter -1].split_whitespace().collect::<Vec<&str>>();

        let execution_result: Result<usize, InterpreterError> = match op[0]{
            "assign" => if op[2] == "to" {
                let reg1 = get_register(op[1]).ok().expect("BadRegisterError: No register of that name!");
                let reg2 = get_register(op[3]).ok().expect("BadRegisterError: No register of that name!");
                let reg3 = get_register(op[5]).ok().expect("BadRegisterError: No register of that name!");



                match op[4] {
                    "plus" => Ok(cpu.assign_plus(reg1, reg2, reg3)),
                    "minus" => Ok(cpu.assign_minus(reg1, reg2, reg3)),
                }
            } else {Err(InterpreterError::SyntaxError)},


            "immassign" => if op[2] == "to" { 
                match op.len() {
                    6 => {
                        let reg1 = get_register(op[1]).ok().expect("BadRegisterError: No register of that name!");
                        let reg2 = get_register(op[3]).ok().expect("BadRegisterError: No register of that name!");
                        let imm: i8 = op[5].parse::<i8>().expect("Invalid immediate size");
                        if op[4] == "plus" {
                            Ok(cpu.immassign_plus(reg1, reg2, imm))
                        } else {
                            Err(InterpreterError::SyntaxError)
                        }
                    },
                    4 => {},
                    _ => Err(InterpreterError::SyntaxError),
                }
            } else {Err(InterpreterError::SyntaxError)},
            "jump" => {},
            "syscall" => {},

            _ => Err(InterpreterError::SyntaxError)
        };

        if op[0] == "jump" {
            line_counter += execution_result
        }

        line_counter += 1;
    }


    println!("---Program finished running (Exit code: 0)---");
}
