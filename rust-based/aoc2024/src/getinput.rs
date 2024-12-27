
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

pub main(){
    let project_path="/Users/onine/git/drfaux-adventofcode/rust-based"
    let src_path="/src"
    let day1_path="/day1"
    let day1_small_input=format!("{}/smallinput.txt",day1_path)
    println!("{} exists? {}",day1_small_input,exists(day1_small_input))
}