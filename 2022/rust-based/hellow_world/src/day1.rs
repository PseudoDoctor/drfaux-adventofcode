use std::{
    fs::{self, metadata, File},
    io::{BufRead, BufReader, Read},
    str,
    cmp,
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
        _ => ()
    }
    data
}
fn main() {
    println!("Hello, world!");
    let _small_input = "day1/smallinput.txt";
    let _regular_input = "day1/input.txt";
    // println!("{}", exists(_regular_input));
    let data = dump_file(_regular_input);
    let top = max_elf_finder(data);
    let mut max = 0;
    let mut all = 0;
    for t in top {
        max = cmp::max(max,t);
        all += t;
    }
    println!("Max {}",max);
    println!("Top three total {}",all);
}

fn max_elf_finder(_data: Vec<u8>) -> Vec<i32> {
    let mut was_digit = false;
    // let mut max = 0;
    let mut elf = 0;
    let mut buf: Vec<u8> = vec![];
    let mut top: Vec<i32> = Vec::new();
    for b in _data {
        match b {
            b'\n' => {
                // println!("end of line");
                if was_digit {
                    // buf's digits are an int, what is that int?
                    let fub = buf.clone(); // clone the buffer so it can be parsed without compiler error
                    // Copied from internet
                    let s = String::from_utf8(fub)
                        .map_err(|non_utf8| {
                            String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
                        })
                        .unwrap();
                    // cast string to int
                    let i: i32 = s.parse().unwrap();
                    // print!("{}", i);
                    // add to elf total
                    elf += i;
                    // print!("{}", elf);
                } else {
                    // part 2
                    top.push(elf);
                    if top.len()>3 {
                        // sort and drop min value
                        top.sort();
                        top.reverse();
                        top.pop();
                    }
                    // reset elf
                    elf = 0;
                }
                was_digit = false;
                buf.clear();
            }
            b'0'..=b'9' => {
                was_digit = true;
                // add digit to buffer
                buf.push(b);
                // print!("digit");
            }
            _ => println!("Unhandled character"),
        }
    }
    top
}
