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

#[derive(Debug, Clone)]
struct CMD {
    working: String,
    command: String,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct DirFil {
    name: String,
    parent: String,
    size: i32,
    child_dirs: Option<Vec<String>>
}

fn main() {
    println!("Hello, world!");
    let _small_input = "day7/smallinput.txt";
    let _regular_input = "day7/input.txt";
    let data = dump_file(_small_input);
    let mut dirs: Vec<DirFil> = Vec::new();
    dirs.push(DirFil{name:String::from("/"),parent:String::from("/"),child_dirs:None,size:-1});
    let mut fils: Vec<DirFil> = Vec::new();
    let mut line: Vec<u8> = Vec::new();
    let mut pwd: String = String::from("");
    let mut dir_sizes: HashMap<DirFil, i32> = HashMap::new();
    for b in data {
        if b == b'\n' {
            let line_chars = line.iter().map(|q| *q as char).collect::<Vec<_>>();
            let line_string = String::from_iter(line_chars.clone());
            // process line
            match line_chars[0] {
                '$' => {
                    print!("CMD");
                    let cm = &line_string[2..];
                    let cmd = CMD {
                        working: pwd.clone(),
                        command: cm.to_string(),
                    };
                    if cmd.command.contains("cd ") {
                        let dest_dir = &cm[3..];
                        println!("Change Directory to '{}'", dest_dir);
                        if (dest_dir == "..") {
                            // println!("Need to find parent directory of '{}'",pwd);
                            // let dirs2 = dirs.clone();
                            // let cwd = dirs2.into_iter().find(|x|x.name == pwd).unwrap();
                            let cwd = get_current_working_directory(dirs.clone(), pwd);
                            println!("Current Working Directory {:?}", cwd);
                            pwd = cwd.parent;
                        } else if dest_dir == "/" {
                            println!("Changing to root");
                            pwd = dest_dir.to_string();
                        } else {
                            pwd = dest_dir.to_string();
                        }
                    } else if cmd.command.contains("ls") {
                        println!("Listing directory '{}'", pwd);
                    }
                }
                'd' => {
                    print!("DIR");
                    let parent = pwd.clone();
                    let tmp_name = &line_string[4..];
                    let name = tmp_name.to_string();
                    let dir = DirFil { name, parent, child_dirs: None, size: -1 };
                    println!("Adding Dir {:?}", dir);
                    dirs.push(dir);
                }
                '0'..='9' => {
                    print!("FIL");
                    let parent = pwd.clone();
                    let re = Regex::new(r"(\d+) (.*)").unwrap();
                    let mut size: i32 = 0;
                    let mut name: String = String::from("");
                    for cap in re.captures_iter(&line_string) {
                        size = cap.get(1).unwrap().as_str().parse().unwrap();
                        name = cap.get(2).unwrap().as_str().to_string();
                    }
                    let fil = DirFil { name, parent, size, child_dirs: None };
                    println!("Adding File {:?}", fil);
                    fils.push(fil);
                }
                _ => {
                    print! {"ERR"}
                }
            }
            // clear line
            line.clear();
        } else {
            line.push(b)
        }
    }
    // Finished parsing input
    // Now calculate sizes
    // Each dir's size, without regard for the presence of children
    for dir in dirs.clone() {
        let mut tmp_size: i32 = 0;
        for fil in &fils {
            if fil.parent == dir.name {
                tmp_size += fil.size;
            }
        }
        println!("Dir {:?} Size {}", dir, tmp_size);
        dir_sizes.insert(dir, tmp_size);
        // TODO: Add children dirs to parent dirs. Loop, find children? I dunno
    }
    let mut adjusted_dir_sizes: HashMap<DirFil, i32> = HashMap::new();
    for dir_map in dir_sizes.clone() {
        let child_dirs = child_dirs(dirs.clone(), dir_map.0);
        if( child_dirs.len() > 0){
            // TODO: How deep do we go?
        }
    }
    println!("Dirs sizes {:?}", dir_sizes.clone());
    println!("");
}

fn get_current_working_directory(_dirs: Vec<DirFil>, _pwd: String) -> DirFil {
    let cwd = _dirs.into_iter().find(|x| x.name == _pwd).unwrap();
    cwd
}

fn child_dirs(_dirs: Vec<DirFil>, _dir: DirFil) -> Vec<DirFil> {
    let mut child_dirs:Vec<DirFil> = Vec::new();
    for dir in _dirs {
        if dir.parent == _dir.name {
            child_dirs.push(dir);
        }
    }
    child_dirs
}
