


use std::{collections::HashMap, env::args, fs, io::{Write, stdin, stdout}, process::exit};

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
#[allow(clippy::enum_variant_names, dead_code)]
enum InterpreterError{
    NotAlnumError,
    SyntaxError,
    BadRegisterError,
    BadImmediateError,
    BadSyscallError,
    AsciiError,

}

//Emulate CPU
#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
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
    fn assign_plus(&mut self, reg1: Register, reg2: Register, reg3: Register) -> i16 {
        if reg1 != Register::Zero {
            *self.registers.get_mut(&reg1).unwrap() = self.registers.get(&reg2).unwrap().wrapping_add(*self.registers.get(&reg3).unwrap());
        }
        0
    }
    fn assign_minus(&mut self, reg1: Register, reg2: Register, reg3: Register) -> i16 {
        if reg1 != Register::Zero {
            *self.registers.get_mut(&reg1).unwrap() = self.registers.get(&reg2).unwrap().wrapping_sub(*self.registers.get(&reg3).unwrap());
        }
        0
    }

    //immediate instructions, 2 registers
    fn immassign_plus(&mut self, reg1: Register, reg2: Register, imm: i8) -> i16 {
        if check_range_i5(imm) {
            if reg1 != Register::Zero {
                *self.registers.get_mut(&reg1).unwrap() = self.registers.get(&reg2).unwrap().wrapping_add_signed(imm.into());
            }
            0
        } else {
            println!("BadImmediateError in instruction \"immassign plus\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");
            exit(2);
        }
    }
    fn jump_if_eq(&mut self, reg1: Register, reg2: Register, imm: i8) -> i16 {
        if check_range_i5(imm) {
            if self.registers.get(&reg1) == self.registers.get(&reg2) {
                imm.into()
            } else {0}
        } else {
            println!("BadImmediateError in instruction \"jump if equals\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");
            exit(2);
        }
    }
    fn jump_if_greater(&mut self, reg1: Register, reg2: Register, imm: i8) -> i16 {
        if check_range_i5(imm) {
            if self.registers.get(&reg1) > self.registers.get(&reg2) {
                imm.into()
            } else {0}
        } else {
            println!("BadImmediateError in instruction \"jump if greaterthan\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");
            exit(2);
        }
    }

    // Immediate instructions, 1 register
    fn immassign_imm(&mut self, reg1: Register, imm: u16) -> i16 {
        if check_range_u9(imm) {
            if reg1 != Register::Zero {
                *self.registers.get_mut(&reg1).unwrap() = imm;
            }
            0
        } else {
            println!("BadImmediateError in instruction \"immassign reg to imm\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");

            exit(2);
        }
    }
    fn syscall(&mut self, reg1: Register, imm: u16) -> i16 {
        if check_range_u9(imm) {
            match imm {
                // 0: ReadInt > reg1
                0 => {
                    let mut read_buf = String::new();
                    stdin().read_line(&mut read_buf).ok().unwrap();
                    *self.registers.get_mut(&reg1).unwrap() = read_buf.trim().parse::<u16>().ok().unwrap();

                },
                // 1: PrintUint
                1 => {
                    print!("{}", *self.registers.get(&reg1).unwrap()); 
                    stdout().flush().unwrap()
                }

                // 2: PrintInt
                2 => {
                    let print_buf = *self.registers.get(&reg1).unwrap();
                    print!("{}", i16::from_ne_bytes(print_buf.to_ne_bytes()));
                    stdout().flush().unwrap();
                }

                // 3: PrintChar
                3 => {
                    if *self.registers.get(&reg1).unwrap() <= 127 {
                        let print_buf = *self.registers.get(&reg1).unwrap() as u8 as char;
                        print!("{}", print_buf);
                        stdout().flush().unwrap();
                    } else {
                        println!("ASCIIError in instruction \"syscall 3\": Attempted to print invalid ASCII character");
                        println!("---Program finished running (Exit code: 4)---");
                        exit(4)
                    }
                }

                // 4: Exit(0)
                4 => {
                    println!("---Program finished running (Exit code: 0)---");
                    exit(0)

                }

                // 5: Exit (reg1)
                5 => {
                    println!("---Program finished running (Exit code: {})---", *self.registers.get(&reg1).unwrap());
                    exit(*self.registers.get(&reg1).unwrap() as i32)
                }
                
                _ => {
                    println!("BadSyscallError in instruction \"syscall {imm}\": Invalid system call code");
                    println!("---Program finished running (Exit code: 3)---");
                    exit(3);
                }

            }
            0
        } else {
            println!("BadImmediateError in instruction \"syscall\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");
            exit(2);
        }
    }

    //Immediate instructions, no register
    fn jump(&mut self, imm: i16) -> i16 {
        if check_range_i13(imm) {
            imm
        } else {
            println!("BadImmediateError in instruction \"jump imm\": Immediate out of range");
            println!("---Program finished running (Exit code: 2)---");
            exit(2);
        }
    }

    
}

//System instructions here

// Functions that check if an immediate is within range (Rust only has 16 and 8 bit integers, not 5, 9 and 13 bits as I need)
fn check_range_i5(int: i8) -> bool {
    (int >= -16) || (int <= 15)
}
fn check_range_u9(int: u16) -> bool {
    int <= 511
}
fn check_range_i13(int: i16) -> bool {
    (int >= -4096) || (int <= 4095)
}

//Function that returns a Register from a register name in string form
fn get_register(regname: &str) -> Result<Register, InterpreterError> {
    match regname {
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
    let operations = match *(file_path.split('.').collect::<Vec<_>>().last().unwrap()) == "az09" {
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
    
    let mut line_counter: usize= 1;
    while line_counter <= statements.len() {
        let op = statements[line_counter -1].split_whitespace().collect::<Vec<&str>>();

        let execution_result: Result<i16, InterpreterError> = match op[0]{
            "assign" => if op[2] == "to" {
                let reg1 = get_register(op[1]).expect("BadRegisterError: No register of that name!");
                let reg2 = get_register(op[3]).expect("BadRegisterError: No register of that name!");
                let reg3 = get_register(op[5]).expect("BadRegisterError: No register of that name!");



                match op[4] {
                    "plus" => Ok(cpu.assign_plus(reg1, reg2, reg3)),
                    "minus" => Ok(cpu.assign_minus(reg1, reg2, reg3)),
                    _ => Err(InterpreterError::SyntaxError),
                }
            } else {Err(InterpreterError::SyntaxError)},


            "immassign" => if op[2] == "to" { 
                match op.len() {
                    6 => {
                        let reg1 = get_register(op[1]).expect("BadRegisterError: No register of that name!");
                        let reg2 = get_register(op[3]).expect("BadRegisterError: No register of that name!");
                        let imm: i8 = op[5].parse::<i8>().expect("Invalid immediate size");
                        if op[4] == "plus" {
                            Ok(cpu.immassign_plus(reg1, reg2, imm))
                        } else {
                            Err(InterpreterError::SyntaxError)
                        }
                    },
                    4 => {
                        let reg1 = get_register(op[1]).expect("BadRegisterError: No register of that name!");
                        let imm: u16 = op[3].parse::<u16>().expect("Invalid immediate size");
                        Ok(cpu.immassign_imm(reg1, imm))
                        },
                    _ => Err(InterpreterError::SyntaxError),
                }
            } else {Err(InterpreterError::SyntaxError)},
            "jump" => {
                match op.len() {
                    6 => {
                        if op[2] == "if" {
                            let reg1 = get_register(op[3]).expect("BadRegisterError: No register of that name!");
                            let reg2 = get_register(op[5]).expect("BadRegisterError: No register of that name!");
                            let imm: i8 = op[1].parse::<i8>().expect("Invalid immediate size");


                            match op[4] {
                                "equals" => Ok(cpu.jump_if_eq(reg1, reg2, imm)),
                                "greaterthan" => Ok(cpu.jump_if_greater(reg1, reg2, imm)),
                                _ => Err(InterpreterError::SyntaxError),
                            }
                        } else {
                            Err(InterpreterError::SyntaxError)
                        }
                    }

                    2 => {
                        let imm: i16 = op[1].parse::<i16>().expect("Invalid immediate size");
                        Ok(cpu.jump(imm))
                    },
                    _ => Err(InterpreterError::SyntaxError)
                }
            },
            "syscall" => {
                let reg1 = get_register(op[2]).expect("BadRegisterError: No register of that name!");
                let imm: u16 = op[1].parse::<u16>().expect("Invalid immediate size");
                Ok(cpu.syscall(reg1, imm))
            },

            _ => Err(InterpreterError::SyntaxError)
        };

        
        let jump_number: i16 = match execution_result {
            Ok(_) => {execution_result.ok().unwrap()},
            Err(error) => {
                println!("{error:?} at instruction number {line_counter}, registers dumped ({cpu:?})");
                println!("---Program finished running (Exit code: 1)---");
                exit(1);
            }
        };
        
        if op[0] == "jump" {
            let abs_jump_number: u16 = jump_number.abs().try_into().unwrap();

            if jump_number >= 0 {
            line_counter += abs_jump_number as usize;
            } else {
                line_counter -= abs_jump_number as usize;
            }
        }
        line_counter += 1;
    }


    println!("---Program finished running (Dropped off bottom)---");
}
