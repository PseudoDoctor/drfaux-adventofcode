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
    let data = dump_file(_small_input);
    let mut buffer: Vec<char> = Vec::new();
    let mut comparts: Vec<i32> = Vec::new();
    for b in data {
        if b == b'\n' {
            comparts.push(compart_priority(buffer.clone()));
            buffer.clear();
        } else {
            buffer.push(b as char);
        }
    }
    for c in comparts {
        println!("Compart {}", c);
    }
}

fn compart_priority(_buffer: Vec<char>) -> i32 {
    let mut p: i32 = -2;
    for c in _buffer {
        println!("Asking about {} and got {}", c, prioity(c))
    }
    p
}

fn prioity(_char: char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let mut p: i32 = -2;
    match _char as u8 {
        b'a'..=b'z' => p = ((_char as u8) - b'a' + 1) as i32,
        b'A'..=b'Z' => p = ((_char as u8) - b'A' + 27) as i32,
        _ => {
            println!("Unhandled: {}", _char);
            p = -1
        }
    }
    println!("Char {} has priority {}", _char, p);
    p
}
