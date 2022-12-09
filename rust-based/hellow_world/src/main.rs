// mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 7
// bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
// nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

use regex::Regex;
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
    let _small_input = "day6/smallinput.txt";
    let _regular_input = "day6/input.txt";
    let data = dump_file(_regular_input);
    let mut line: Vec<u8> = Vec::new();
    for b in data {
        if b == b'\n' {
            let line_chars = line.iter().map(|q| *q as char).collect::<Vec<_>>();
            let line_string = String::from_iter(line_chars);
            // process line
            let marker: i32 = process(line.clone());
            println!("Marker at {} for {}", marker, line_string);
            line.clear();
        } else {
            line.push(b)
        }
    }
}

fn process(_line: Vec<u8>) -> i32 {
    // first time the last four characters were different
    let mut four_u8s: Vec<u8> = Vec::new();
    let mut match_count: i32 = -1;
    for b in _line {
        if four_u8s.len() < 5 {
            four_u8s.push(b);
        } else {
            // do any of the four match?
            // a match or switch wouldn't work...
            // If/Thens should work... four charactesr require 6 comparisons
            if ( four_u8s[0] == four_u8s[1]
                || four_u8s[0] == four_u8s[2]
                || four_u8s[0] == four_u8s[3] 
                // First  ^
                // SEcond  v
                || four_u8s[1] == four_u8s[2] 
                || four_u8s[1] == four_u8s[3] 
                // SEcond ^
                // Third and fourth v
                || four_u8s[2] == four_u8s[3] ) {
                // One of the last four characters matched, delete the oldest and move on
                four_u8s.reverse();
                let _old = four_u8s.pop().unwrap() as char;
                // println!("Removing {}",_old);
                four_u8s.reverse();
                four_u8s.push(b);
            } else {
                // FOUND THE FIRST NON-REPEATING
                return match_count;
            }
        }
        match_count += 1;
    }
    -2
}
