
use std::{
    fs::{metadata, File},
    io::Read,
    str,
    path::PathBuf,
};



/// Does path exist? (Uses &str and passes through std::fs::metadata)
pub fn exists_str(path: &str) -> bool {
    if metadata(path).is_ok() {
        let md = metadata(path).unwrap();
        println!("is dir: {}", md.is_dir());
        println!("is file: {}", md.is_file());
        return true;
    }
    false
}

pub fn testday1(){
    // let path = env::current_dir();
    // path.push("day1");
    // println!("The current directory is {}", path.);
    let project_path="/Users/onine/git/drfaux-adventofcode/rust-based/aoc2024";
                    let _src_path="/src";
    let day1_path="/day1";
    let day1_small_input=format!("{}{}/smallinput.txt",project_path,day1_path);
    println!("{} exists? {}",day1_small_input,exists_str(&day1_small_input));
    let day1_small_input2 = PathBuf::from(format!(".{}/smallinput.txt",day1_path));
    println!("{} exists? {}",day1_small_input2.display(),day1_small_input2.exists());
    let input_vec = get_input(1,true);
    let s = String::from_utf8(input_vec).expect("Found invalid UTF-8");
    println!("Input:\n{}",s);
    
}
/// Get day's input.txt and returns as a Vec<u8> (Vector of Unicode-8 regardless of printability)
pub fn get_input(day_number: i8,is_small: bool) -> Vec<u8>{
    assert!(day_number > 0 || day_number < 26);
    let input;
    let small;
    if is_small {
        small = "small";
    } else {
        small = "";
    }
    input = format!("./day{}/{}input.txt",day_number,small);
    // println!("Looking for input: {}",input);
    let input_path = PathBuf::from(input);
    assert!(input_path.exists());
    let data = std::fs::read(input_path);
    return data.unwrap()
}
/// Convert Vec<u8> to String (See get_input which uses std::fs::read )
pub fn vec2string(v: Vec<u8>) -> String {
    let s = String::from_utf8(v).expect("Found invalid UTF-8");
    return s
}

/// Legacy get_input alternative
pub fn dump_file(path: &str) -> Vec<u8> {
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

// Leftover tutorial stuff
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
