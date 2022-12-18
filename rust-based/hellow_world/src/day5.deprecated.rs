use std::{
    cmp,
    collections::HashMap,
    fs::{self, metadata, File},
    io::{BufRead, BufReader, Read},
    str,
};

fn dump_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
    data
}

enum LineType {
    CrateRow,
    ColumnNumbers,
    Empty,
    MoveInstruction,
    Invalid,
}

struct Stack {
    column: i32,
    crates: Vec<char>,
}

impl Copy for Stack {}

impl Clone for Stack {
    fn clone(&self) -> Stack {
        *self
    }
}

fn main() {
    println!("Hello, world!");
    let _small_input = "day5/smallinput.txt";
    let _regular_input = "day5/input.txt";
    let data = dump_file(_small_input);
    println!("");
    let mut big_buffer: Vec<u8> = Vec::new();
    let mut stacks: Vec<Stack> = Vec::new();
    let mut stack_temp: Vec<char> = Vec::new();
    let mut line: Vec<u8> = Vec::new();
    let crate_width = 4;
    for d in data {
        big_buffer.push(d);
        line.push(d);
        if d == b'\n' {
            if line.len() > 1 {
                // char was a newline, but no other characters exist,
                //  therefore it should be a LineType::Empty
                let l: LineType = which_line_type(line.clone());
                match l {
                    LineType::CrateRow => {
                        // process crates into stacks
                        let mut crate_temp: Vec<char> = Vec::new();
                        let mut index_tmp = 1;
                        let mut column_count = 1;
                        loop {
                            if(index_tmp > line.len()){break;}
                            let c: char = line[index_tmp] as char;
                            print!(" crate at {} is {}",index_tmp,c);
                            crate_temp.push(c);
                            index_tmp += crate_width;
                            column_count += 1;
                        }
                        // more proccesing
                        // crate_temp is now the crates on the row, 
                        //  As of now, crate_temp.pop is the right-most crate, 
                        //   which corresponds to column_count
                        // Reverse and then loop
                        crate_temp.reverse();
                        if stacks.len() == 0 {
                            for column in 1..=column_count{
                                let crate_char = vec![crate_temp.pop().unwrap()];
                                let stack_Stack = Stack {column:column,crates:crate_char};
                                stacks.push(stack_Stack);
                            }
                        } else {
                            // prepend existing stacks
                            for column in 1..=column_count{
                                let mut s3 = Stack{column:1,crates:Vec::new()};
                                let mut i3: usize = 0;
                                for (i,s) in stacks.iter().enumerate() {
                                    if s.column == column{
                                        let s2 = s.clone();
                                        s3 = initialize_stack(s2,crate_temp.pop().unwrap());
                                        i3 = i;
                                    }
                                }
                                stacks[i3]=s3;
                            }
                        }
                        println!("End crate row");
                    },
                    LineType::ColumnNumbers => {},
                    LineType::Empty => {},
                    LineType::MoveInstruction => {},
                    LineType::Invalid => {}
                }
                //process
                line.clear();
            } else {
                // gap line, remaining lines should be rows
                println!("newline Row");
                line.clear();
            }
        }
    }
}

fn which_line_type(_line: Vec<u8>) -> LineType {
    let char0 = _line[0] as char;
    let char1 = _line[1] as char;
    struct ZeroOne(char, char);
    print!("Checking {}{} - ", char0, char1);
    match ZeroOne(char0, char1) {
        ZeroOne(' ', ' ') | ZeroOne('[', _) => {
            println!("Crate Row");
            return LineType::CrateRow;
        }
        ZeroOne(' ', _) => {
            println!("Column Row");
            return LineType::ColumnNumbers;
        }
        ZeroOne('m', 'o') => {
            println!("Move Row");
            return LineType::MoveInstruction;
        }
        _ => {
            println!("Unhandled {}{}", char0, char1);
            return LineType::Invalid;
        }
    }
}

fn get_column_count(_line:Vec<u8>)->i32{
    (_line.len() / 4) as i32
}

fn initialize_stack(_stack:Stack,_crate:char)->Stack{
    // During initialization, stacks(columns) are given in rows of crates
    // This function focuses on a single Stack and a single Crate being added below any existing crates
    // Crates can be empty ' ' and must be populated during initialization
    let column = _stack.column;
    let mut crates = _stack.crates;
    // Assume crates.push() will add to the "bottom" of the stack, so we need to reverse it, push, and reverse again
    crates.reverse();
    crates.push(_crate);
    crates.reverse();
    Stack {column:column,crates:crates}
}

fn move_crate(_stacks:Vec<Stack>,_count: i32,_from: i32,_to:i32)->Vec<Stack>{
    let mut new_stacks: Vec<Stack> = Vec::new();
    new_stacks
}
