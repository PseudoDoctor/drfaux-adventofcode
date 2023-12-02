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
    let _small_input = "day4/smallinput.txt";
    let _regular_input = "day4/input.txt";
    let data = dump_file(_regular_input);
    let mut buf: Vec<u8> = Vec::new();
    let mut contained_count: i32 = 0;
    let mut overlapped_count: i32 = 0;
    for d in data {
        if d == b'\n' {
            let range = parse_ranges(buf.clone());
            let contained = fully_contained(range.clone());
            let overlaps = overlapped(range.clone());
            // println!("contained {}", contained);
            if contained {
                contained_count += 1;
            };
            if overlaps {
                overlapped_count += 1;
            };
            buf.clear();
        } else {
            buf.push(d);
        }
    }
    println!("Total Contained {}", contained_count);
    println!("Overlapped {}", overlapped_count);
}

fn parse_ranges(_data: Vec<u8>) -> Vec<i32> {
    let mut thingy: Vec<i32> = Vec::new();
    let mut i_buf: Vec<i32> = Vec::new();
    let mut i_b: i32 = 0;
    for d in _data {
        match d {
            b'0'..=b'9' => {
                let c: char = d as char;
                let i: i32 = (c as u8 - b'0').into();
                i_buf.push(i);
                // print!("digit{} ", d as char);
            }
            b'-' => {
                let mut place = 1;
                i_buf.reverse();
                for i in &i_buf {
                    i_b += i * place;
                    place = place * 10;
                }
                thingy.push(i_b);
                // print!("dash{} ", d as char);
                // println!("int{} ", i_b);
                i_buf.clear();
                i_b = 0;
            }
            b',' => {
                let mut place = 1;
                i_buf.reverse();
                for i in &i_buf {
                    i_b += i * place;
                    place = place * 10;
                }
                thingy.push(i_b);
                // print!("comma{} ", d as char);
                // println!("int{} ", i_b);
                i_buf.clear();
                i_b = 0;
            }
            _ => {
                print!("unhandled{} ", d as char);
            }
        }
    }
    let mut place = 1;
    i_buf.reverse();
    for i in &i_buf {
        i_b += i * place;
        place = place * 10;
    }
    thingy.push(i_b);

    // println!("int{} ", i_b);
    i_buf.clear();
    i_b = 0;
    // println!("");
    thingy
}

fn overlapped(_ranges: Vec<i32>) -> bool {
    if _ranges[0] <= _ranges[2] && _ranges[1] >= _ranges[2] {
        return true;
    } else if _ranges[2] <= _ranges[0] && _ranges[3] >= _ranges[0] {
        return true;
    } else {
        return false;
    }
}

fn fully_contained(_ranges: Vec<i32>) -> bool {
    if _ranges.len() == 4 {
        print!("{}-{},{}-{} ", _ranges[0], _ranges[1], _ranges[2], _ranges[3]);
        if (_ranges[0] <= _ranges[2] && _ranges[1] >= _ranges[3]) {
            println!("yes right");
            return true;
        } else if (_ranges[2] <= _ranges[0] && _ranges[3] >= _ranges[1]) {
            println!("yes left");
            return true;
        } else {
            println!("no");
            return false;
        }
    } else {
        println!("Not enough thingies");
    }
    false
}
