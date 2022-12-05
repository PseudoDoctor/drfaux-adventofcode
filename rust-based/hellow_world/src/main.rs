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
    println!("{}", exists(_small_input));
    let data = dump_file(_regular_input);
    let mut buffer: Vec<char> = Vec::new();
    let mut single_priorities: Vec<i32> = Vec::new();
    for b in data {
        if b == b'\n' {
            single_priorities.push(compart_priority(buffer.clone()));
            buffer.clear();
        } else {
            buffer.push(b as char);
        }
    }
    let mut sum: i32 = 0;
    for c in single_priorities {
        sum += c;
    }
    println!("Sum {}",sum);
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
    let m = get_matching(left,right);
    _p = get_priority(m);
    println!("{}' have '{}' in common with priority '{}'",output,m,_p);
    _p
}

fn get_priority(_char: char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let mut p: i32 = -2;
    match _char as u8 {
        // range a-z and A-Z
        // take the char's u8 value and subtract b'a' from it, then add 1 or 27
        b'a'..=b'z' => p = ((_char as u8) - b'a' + 1) as i32,
        b'A'..=b'Z' => p = ((_char as u8) - b'A' + 27) as i32,
        _ => {
            println!("Unhandled: {}", _char);
            p = -1
        }
    }
    // println!("Char {} has priority {}", _char, p);
    p
}

fn get_matching(_left: Vec<char>,_right: Vec<char>) -> char{
    for l in &_left {
        for r in &_right {
            if l == r { return *l}
        }
    }
    '-'
}
