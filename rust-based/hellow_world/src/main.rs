// mjqjpqmgbljsphdztnvjfqwrcgsmlb: packet 7 message 19
// bvwbjplbgvbhsrlpgdmjqwftvncz: packet 5 message 23
// nppdvjthqldpwncqszvftbrmjlhg: packet 6 message 23
// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: packet 10 message 29
// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: packet 11 message 26

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
    let data = dump_file(_small_input);
    let mut line: Vec<u8> = Vec::new();
    for b in data {
        if b == b'\n' {
            let line_chars = line.iter().map(|q| *q as char).collect::<Vec<_>>();
            let line_string = String::from_iter(line_chars);
            // process line
            let packet_marker: i32 = get_start_of_packet(line.clone());
            let message_marker: i32 = get_start_of_message(line.clone());
            println!("Marker Packet {} Message {} for {}",packet_marker, message_marker, line_string);
            line.clear();
        } else {
            line.push(b)
        }
    }
}

fn get_start_of_packet(_line: Vec<u8>) -> i32 {
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

fn get_start_of_message(_line: Vec<u8>) -> i32 {
    // first time the last fourteen characters were different

    let mut fourteen: Vec<u8> = Vec::new();
    let mut match_count: i32 = -1;
    for b in _line {
        if fourteen.len() < 15 {
            fourteen.push(b);
        } else {
            let fourteen_chars = fourteen.iter().map(|q| *q as char).collect::<Vec<_>>();
            let fourteen_string = String::from_iter(fourteen_chars);
            println!("Last fourteen {}",fourteen_string);
            // If/Thens for four charactesr require 6 comparisons
            if ( fourteen[0] == fourteen[1]
                || fourteen[0] == fourteen[2]
                || fourteen[0] == fourteen[3] 
                // First  ^
                // SEcond  v
                || fourteen[1] == fourteen[2] 
                || fourteen[1] == fourteen[3] 
                // SEcond ^
                // Third and fourth v
                || fourteen[2] == fourteen[3] ) {
                // One of the last four characters matched, delete the oldest and move on
                fourteen.reverse();
                let _old = fourteen.pop().unwrap() as char;
                // println!("Removing {}",_old);
                fourteen.reverse();
                fourteen.push(b);
            } else if(
                // What about 14 characters? 105 comparisons?
                // This needs a loop. A recursive loop?
                // I don't think I can code this myself
                // let's use the Contains function (if it exists)
                false
            ) {

            }
            {
                // FOUND THE FIRST NON-REPEATING
                return match_count;
            }
        }
        match_count += 1;
    }
    -2
}