use std::io::Read;
use std::fs;

pub fn exists(path: &str) -> bool {
    if fs::metadata(path).is_ok() {
        let md = fs::metadata(path).unwrap();
        println!("is dir: {}", md.is_dir());
        println!("is file: {}", md.is_file());
        return true;
    }
    false
}
fn main() {
    println!("Hello, world!");
    let _small_input = "day1/smallinput.txt";
    let _regular_input = "day1/input.txt";
    // println!("{}",exists(_regular_input));
    // dump_file(_regular_input);
    read_file_line_by_line(_small_input);
    loop_de_loop();
}

fn dump_file(path: &str) {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
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