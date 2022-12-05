use std::{
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
    let max = max_elf_finder(data);
    println!("Max {}",max);
}

fn max_elf_finder(_data: Vec<u8>) -> i32 {
    let mut was_digit = false;
    let mut max = 0;
    let mut elf = 0;
    let mut buf: Vec<u8> = vec![];
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
                    // end of elf, is it max?
                    if elf > max {
                        max = elf;
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
    max
}



fn loop_de_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; 512];

    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;

        do_something(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

fn do_something(_data: &[u8]) {
    let mut was_digit = false;
    let mut max = 0;
    let mut elf = 0;
    let mut buf: Vec<u8> = vec![];
    for b in _data {
        match *b {
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
                    // end of elf, is it max?
                    if elf > max {
                        max = elf;
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
                buf.push(*b);
                // print!("digit");
            }
            _ => println!("Unhandled character"),
        }
    }
    println!("Done processing, max calories: {}", max);
    // println!("{}",std::str::from_utf8(_data).unwrap());
}

fn list_utf8_codes() {
    let newline = b'\n';
    println!("{:x?}", newline);
    let newline = b"\n";
    println!("{:x?}", newline);
    let one = b"1";
    println!("{:x?}", one);
}
