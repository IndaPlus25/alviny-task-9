


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
    ParseError,

}
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
    fn assign_plus(&mut self, reg1: Register, reg2: Register, reg3: Register) {
        todo!()
    }
    fn assign_minus(&mut self, reg1: Register, reg2: Register, reg3: Register) {
        todo!()
    }

    //immediate instructions, 2 registers
    fn immassign_plus(&mut self, reg1: Register, reg2: Register, imm: i8) {
        todo!()
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

        let execution_result: Result = match op[0]{
            _ => 
        }


        line_counter += 1;
    }


    println!("---Program finished running (Exit code: 0)---");
}
