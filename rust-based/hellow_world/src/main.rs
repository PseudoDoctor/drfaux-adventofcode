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
fn main() {
    println!("Hello, world!");
    let _small_input = "day3/smallinput.txt";
    let _regular_input = "day3/input.txt";
    let _tiny_input = "day3/tinyinput.txt";
    // println!("{}", exists(_tiny_input));
    let data = dump_file(_regular_input);
    let mut elfs: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    for b in data {
        if b == b'\n' {
            elfs.push(buffer_2_string(buffer.clone()));
            buffer.clear();
        } else {
            buffer.push(b);
        }
    }
    // part 1
    let mut sum1: i32 = 0;
    for elf in elfs.clone() {
        let len = elf.len();
        let lenhalf = len/2;
        let left = String::from(&elf[..lenhalf]);
        let right = String::from(&elf[lenhalf..]);
        // println!("left {} right {}",left,right);
        // let common = common_characters(left.clone(),right.clone());
        let common_single = common_character_single(left.clone(),right.clone());
        // println!("Common character for {} and {} is {}",left,right,common_single.clone());
        sum1 += get_priority(common_single);
    }
    println!("Sum1 {}",sum1);
    // part 2
    let mut sum2: i32 = 0;
    loop {
        if elfs.len() >= 3 {
            let one = elfs.pop().unwrap();
            let two = elfs.pop().unwrap();
            let three = elfs.pop().unwrap();
            let common_onetwo = common_characters(one.clone(), two);
            let common_onetwothree = common_characters(common_onetwo, three);
            let common = common_character_single(one.clone(), common_onetwothree.clone());
            // println!( "Common char {}",common)
            sum2 += get_priority(common);
        } else {
            if elfs.len() == 0 {
                break;
            } else {
                println!("Bad state ");
                break;
            }
        }
    }
    println!("Sum2 {}",sum2);
    
}

fn buffer_2_string(_buffer: Vec<u8>) -> String {
    let mut s = String::from("");
    for b in _buffer {
        s.push(b as char);
    }
    s
}

fn common_characters(_a: String,_b:String)->String{
    let mut _s = String::from("");
    for (i,a) in _a.bytes().enumerate() {
        if _b.contains(a as char) {
            _s.push(a as char);
            // println!("{} from left at index {}",a as char,i);
        }
    }
    for (i,b) in _b.bytes().enumerate(){
        if _s.contains(b as char) {
            // println!("{} from right at index {}",b as char, i);
        }
    }
    _s
}

fn common_character_single(_left: String,_right: String) -> char{
    for l in _left.chars() {
        for r in _right.chars() {
            if l == r { return l}
        }
    }
    '-'
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
