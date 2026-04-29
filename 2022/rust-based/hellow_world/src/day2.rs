use std::{
    cmp,
    collections::HashMap,
    fs::{self, metadata, File},
    io::{BufRead, BufReader, Read},
    str,
};
static ROCK: i32 = 1;
static PAPER: i32 = 2;
static SCISSORS: i32 = 3;
static LOSE: i32 = 0;
static DRAW: i32 = 3;
static WIN: i32 = 6;

// Rock defeats Scissors,
// Scissors defeats Paper, and
// Paper defeats Rock.
// If both players choose the same shape, the round instead ends in a draw.
// The score for a single round is the score for the shape you selected
// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
// outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
// A Y
// B X
// C Z
// Round 1: Rock A, choose Paper Z. You win, score won 6 + choose paper 2 = 8.
// Round 2: Paper B, choose Rock X. You lose, score lose 0 + choose rock 1 = 1.
// Round 3: SCISSORS C, choose SCISSORS Z. Draw, score draw 2 + choose SCISSORS 3 = 6

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
    let _small_input = "day2/smallinput.txt";
    let _regular_input = "day2/input.txt";
    // println!("{}", exists(_regular_input));
    let data = dump_file(_small_input);
    let score = rock_paper_scisors(data);
    println!("total score {}",score)
}
fn rock_paper_scisors(_data: Vec<u8>) -> i32 {
    let mut rounds: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    let mut prev: u8 = b'0';
    let mut play: u8 = b'0';
    let mut response: u8 = b'0';
    for b in _data {
        print!("{}-", b);
        if b == b' ' {
            play = prev;
        } else if b == b'\n' {
            response = prev;
            let score = decidepart2(play, response);
            rounds.push(score);
            total += score;
        }
        prev = b;

    }
    for r in rounds {
            println!(
                "Rounds {}",r
            )
    }
    println!("Total {}",total);
    total
}

fn decidepart1(_play: u8, _response: u8) -> i32 {
        // match b {
        //     b'A'|b'X' => print!("Rock"),
        //     b'B'|b'Y' => print!("Paper"),
        //     b'C'|b'Z' => print!("Scis"),
        //     b' ' => print!(" "),
        //     b'\n'=> println!(""),
        //     _ => println!("Unhandled")
        // }

    let mut score: i32 = 0;
    match _play {
        b'A' => {print!("Rock vs ");
            match _response {
            b'X' => {print!("Rock");score += DRAW + ROCK},
            b'Y' => {print!("Paper");score += WIN  + PAPER},
            b'Z' => {print!("Sciscors");score += LOSE + SCISSORS},
            _ => println!("Unhandled")
        }},
        b'B' => {print!("Paper vs ");
             match _response {
            b'X' => {print!("Rock");score += LOSE + ROCK},
            b'Y' => {print!("Paper");score += DRAW + PAPER},
            b'Z' => {print!("Sciscors");score += WIN  + SCISSORS},
            _ => println!("Unhandled")
        }},
        b'C' => {print!("Sciscors vs ");
            match _response {
            b'X' => {print!("Rock");score += WIN + ROCK},
            b'Y' => {print!("Paper");score += LOSE + PAPER},
            b'Z' => {print!("Sciscors");score += DRAW + SCISSORS},
            _ => println!("Unhandled")
        }},
        _ => println!("Unhandled")
    }
    println!(" {}",score);
    score
}

fn decidepart2(_play: u8, _response: u8) -> i32 {
    let mut score: i32 = 0;
    match _play {
        b'A' => {print!("Rock vs ");
            match _response {
            b'X' => {print!(" LOSE Sciscors");score += LOSE + SCISSORS},
            b'Y' => {print!(" DRAW Rock");score += DRAW  + ROCK},
            b'Z' => {print!(" WIN Paper");score += WIN + PAPER},
            _ => println!("Unhandled")
        }},
        b'B' => {print!("Paper vs ");
             match _response {
            b'X' => {print!(" LOSE Rock");score += LOSE + ROCK},
            b'Y' => {print!(" DRAW Paper");score += DRAW + PAPER},
            b'Z' => {print!(" WIN Sciscors");score += WIN  + SCISSORS},
            _ => println!("Unhandled")
        }},
        b'C' => {print!("Sciscors vs ");
            match _response {
            b'X' => {print!(" LOSE Paper");score += LOSE + PAPER},
            b'Y' => {print!(" DRAW Sciscosrs");score += DRAW + SCISSORS},
            b'Z' => {print!(" WIN Rock");score += WIN + ROCK},
            _ => println!("Unhandled")
        }},
        _ => println!("Unhandled")
    }
    println!(" {}",score);
    score
}
