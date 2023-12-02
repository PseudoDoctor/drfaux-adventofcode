use std::{
    cmp,
    collections::HashMap,
    fs::{self, metadata, File},
    io::{BufRead, BufReader, Read},
    str,
};
pub fn exists(path: &str) -> bool {
    if metadata(path).is_ok() {
        let md = metadata(path).unwrap();
        println!("is dir: {}", md.is_dir());
        println!("is file: {}", md.is_file());
        return true;
    }
    false
}

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
fn main() {
    println!("Hello, world!");
    let _small_input = "day3/smallinput.txt";
    let _regular_input = "day3/input.txt";
    let _tiny_input = "day3/tinyinput.txt";
    // println!("{}", exists(_tiny_input));
    let data = dump_file(_tiny_input);
    let mut buffer: Vec<char> = Vec::new();
    let mut single_priorities: Vec<i32> = Vec::new();
    let mut triple_buffer: Vec<Vec<char>> = Vec::new();
    let mut triple_priorities: Vec<i32> = Vec::new();
    for b in data {
        if b == b'\n' {
            triple_buffer.push(buffer.clone());
            single_priorities.push(compart_priority(buffer.clone()));
            buffer.clear();
        } else {
            buffer.push(b as char);
        }
        if triple_buffer.len() == 3 {
            println!("beep1");
            // process these three items
            let one: Vec<char> = triple_buffer.pop().unwrap();
            let two: Vec<char>  = triple_buffer.pop().unwrap();
            let three: Vec<char>  = triple_buffer.pop().unwrap();
            println!("beep2");
            let onetwo: Vec<char> = get_matching_all(one, two);
            let onetwothree: Vec<char> = get_matching_all(onetwo, three);
            println!("beep3");
            if onetwothree.len() == 1 {
                let pthree = get_priority(onetwothree[0]);
                triple_priorities.push(pthree);
            } else {
                println!("PANIC");
            }
            // clear
            triple_buffer.clear();
        }
    }
    let mut sum: i32 = 0;
    for c in single_priorities {
        sum += c;
    }
    let mut sumthree: i32 = 0;
    println!("Each Compartment Sum {}",sum);
    for c in triple_priorities {
        println!("Last three elf common priority: {}",c);
        sumthree += c;
    }
    println!("Group by Three Elf Sum {}",sumthree);
}

fn compart_priority(_buffer: Vec<char>) -> i32 {
    let mut output: String = String::from("Compartments '");
    let mut _p: i32 = -2;
    let len = _buffer.len();
    let newlen = len/2;
    let left: Vec<char> = _buffer[..newlen].to_vec();
    for c in &left{
        output += &String::from(*c);
    }
    output += "' and '";
    let right: Vec<char> = _buffer[newlen..].to_vec();
    for c in &right{
        output += &String::from(*c);
    }
    let m = get_matching_single(left,right);
    _p = get_priority(m);
    println!("{}' have '{}' in common with priority '{}'",output,m,_p);
    _p
}

fn get_priority(_char: char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let mut _p: i32 = -2;
    match _char as u8 {
        // range a-z and A-Z
        // take the char's u8 value and subtract b'a' from it, then add 1 or 27
        b'a'..=b'z' => _p = ((_char as u8) - b'a' + 1) as i32,
        b'A'..=b'Z' => _p = ((_char as u8) - b'A' + 27) as i32,
        _ => {
            println!("Unhandled: {}", _char);
            _p = -1
        }
    }
    // println!("Char {} has priority {}", _char, p);
    _p
}

fn get_matching_single(_left: Vec<char>,_right: Vec<char>) -> char{
    for l in &_left {
        for r in &_right {
            if l == r { return *l}
        }
    }
    '-'
}

fn get_matching_all(_a: Vec<char>,_b: Vec<char>) -> Vec<char> {
    let mut _out: Vec<char> = Vec::new();
    let mut _tmp: Vec<char> = Vec::new();
    for a in &_a {
        for b in &_b {
            // print!("{}-{}",a,b);
            if a == b { 
                _tmp = _out.clone();
                // print!(" == ");
                if _tmp.len() == 0 {
                    // println!("{} addfirst",a);
                    _out.push(*a);
                } else {
                    for o in &_tmp {
                        // println!(" {}?",o);
                        if a != o {
                            _out.push(*a);
                            // println!("{} add",a);
                        }
                    }
                }
                _tmp.clear();
            } else {
                // println!(" != ");
            }
        }
        // println!("");
    }
    let _tmp2 = _out.clone();
    for t in _tmp2 {
        // print!("{} ", t);
    }
    // println!(" end");
    _out
}
