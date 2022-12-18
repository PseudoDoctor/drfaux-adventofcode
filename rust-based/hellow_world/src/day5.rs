use std::{
    cmp,
    collections::HashMap,
    fs::{self, metadata, File},
    io::{BufRead, BufReader, Read},
    str,
};
use regex::Regex;

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

fn main() {
    println!("Hello, world!");
    let _small_input = "day5/smallinput.txt";
    let _regular_input = "day5/input.txt";
    let data = dump_file(_regular_input);
    println!("");
    let mut big_buffer: Vec<u8> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
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
                        let mut column_count = 0;
                        loop {
                            if (index_tmp > line.len()) {
                                break;
                            }
                            let c: char = line[index_tmp] as char;
                            print!(" crate at {} is {}", index_tmp, c);
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
                            // initialize stacks
                            for column in 1..=column_count {
                                let c1 = crate_temp.pop();
                                print!(" foo {:?} bar", c1);
                                let c2 = c1.unwrap();
                                let crate_char = vec![c2];
                                stacks.push(crate_char);
                            }
                        } else {
                            // prepend existing stacks
                            for column in 1..=column_count {
                                stacks[(column - 1)].reverse();
                                let c1 = crate_temp.pop();
                                print!(" foo {:?} bar", c1);
                                let c2 = c1.unwrap();
                                stacks[(column - 1)].push(c2);
                                stacks[(column - 1)].reverse();
                            }
                        }
                        println!(" End crate row");
                    }
                    LineType::ColumnNumbers => {
                        // copy paste from crates
                        let mut column_temp: Vec<char> = Vec::new();
                        let mut index_tmp = 1;
                        let mut column_count = 0;
                        loop {
                            if (index_tmp > line.len()) {
                                break;
                            }
                            let c: char = line[index_tmp] as char;
                            print!(" column at {} is {}", index_tmp, c);
                            column_temp.push(c);
                            index_tmp += crate_width;
                            column_count += 1;
                        }
                        // remove spaces from existing stacks
                        // temporary print existing stack
                        // for c in column_temp {
                        //     // column number index is column - 1
                        //     let i = (c as u8 - '0' as u8) as usize - 1;
                        //     print!(" Looking at column {} ", c);
                        //     let stacks2 = &stacks.clone();
                        //     let s = &stacks2[i];
                        //     print!(" values: {:?}", *s);
                        // }
                        let mut stacks2 = trim_stacks(stacks.clone());
                        // temporary print existing stack2
                        stacks = stacks2;
                        for stack in &stacks {
                            print!(" Stack {:?}", *stack);
                        }

                        println!(" End columns row")
                    }
                    LineType::Empty => {}
                    LineType::MoveInstruction => {
                        let mut count: i32 = 0;
                        let mut from: i32 = 0;
                        let mut to: i32 = 0;
                        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                        let mut line_chars: Vec<char> = Vec::new();
                        for b in &line {
                            let c = *b as char;
                            line_chars.push(c);
                        }
                        let line_string = String::from_iter(line_chars);
                        for cap in re.captures_iter(&line_string){
                            count = cap.get(1).unwrap().as_str().parse().unwrap();
                            from = cap.get(2).unwrap().as_str().parse().unwrap();
                            to = cap.get(3).unwrap().as_str().parse().unwrap();
                            // println!("Move {} From {} to {}",&cap[1],&cap[2],&cap[3]);
                            
                        }
                        println!("Requesting Move {} From {} to {}",count,from,to);
                        let mut stacks2 = move_crates_all_at_once(stacks.clone(),count,from,to);
                        stacks = stacks2;
                        print!("Post move ");
                        for stack in &stacks {
                            print!(" Stack {:?}", *stack);
                        }
                        println!("")
                    }
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
    // finally
    let mut top_of_stacks: Vec<char> = Vec::new();
    let mut stacks2 = stacks.clone();
    for mut stack in stacks2 {
        let c = stack.pop().unwrap();
        top_of_stacks.push(c);
    }
    print!(" top of stacks {:?}", top_of_stacks);
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

fn get_column_count(_line: Vec<u8>) -> i32 {
    (_line.len() / 4) as i32
}

fn move_crates_one_by_one(mut _stacks: Vec<Vec<char>>, _count: i32, _from: i32, _to: i32) -> Vec<Vec<char>> {
    let from_index = (_from - 1) as usize;
    let to_index = (_to - 1) as usize;
    let mut crate_char: char = '0';
    for a in 1..=_count {
        crate_char = _stacks[from_index].pop().unwrap();
        _stacks[to_index].push(crate_char);
        println!("This is move {} which takes char {} from {} to {}",a,crate_char,_from,_to);
    }
    _stacks
}
fn move_crates_all_at_once(mut _stacks: Vec<Vec<char>>, _count: i32, _from: i32, _to: i32) -> Vec<Vec<char>> {
    let from_index = (_from - 1) as usize;
    let to_index = (_to - 1) as usize;
    let mut crate_char: char = '0';
    let mut small_stack:Vec<char> = Vec::new();
    for a in 1..=_count {
        crate_char = _stacks[from_index].pop().unwrap();
        small_stack.push(crate_char);
    }
    print!("Small stack of {} items {:?} taken from {} and put on {}",_count,small_stack,_from,_to);
    for b in 1..=_count {
        crate_char = small_stack.pop().unwrap();
        _stacks[to_index].push(crate_char);
    }
    _stacks
}

fn trim_stacks(mut _stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let stack_count = _stacks.len();
    for si in 0..stack_count {
        let mut pop_count = 0;
        let crate_count = _stacks[si].len();
        for ci in 0..crate_count {
            let c = _stacks[si][ci];
            if c == ' ' {
                pop_count += 1;
            }
        }
        for p in 0..pop_count {
            _stacks[si].pop();
        }
    }
    _stacks
}
