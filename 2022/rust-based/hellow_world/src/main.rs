// mjqjpqmgbljsphdztnvjfqwrcgsmlb: packet 7 message 19
// bvwbjplbgvbhsrlpgdmjqwftvncz: packet 5 message 23
// nppdvjthqldpwncqszvftbrmjlhg: packet 6 message 23
// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: packet 10 message 29
// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: packet 11 message 26

use regex::Regex;
use std::{
    str, fs::File, io::Read,
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
    /// DIR if < 0, otherwise FILE
    size: i32,
    child_fileize: i32,
    child_dirs: Option<Vec<String>>,
    child_dirsize: i32,
}

fn main() {
    println!("Hello, world!");
    let _small_input = "day7/smallinput.txt";
    let _regular_input = "day7/input.txt";
    let data = dump_file(_small_input);
    let mut dirs: Vec<DirFil> = Vec::new();
    dirs.push(DirFil {
        name: String::from("/"),
        parent: String::from("/"),
        child_dirs: None,
        size: -1,
        child_fileize: -1,
        child_dirsize: -1,
    });
    let mut fils: Vec<DirFil> = Vec::new();
    let mut line: Vec<u8> = Vec::new();
    let mut pwd: String = String::from("");
    for b in data {
        if b == b'\n' {
            let line_chars = line.iter().map(|q| *q as char).collect::<Vec<_>>();
            let line_string = String::from_iter(line_chars.clone());
            // process line
            match line_chars[0] {
                '$' => {
                    // print!("CMD");
                    let cm = &line_string[2..];
                    let cmd = CMD {
                        working: pwd.clone(),
                        command: cm.to_string(),
                    };
                    if cmd.command.contains("cd ") {
                        let dest_dir = &cm[3..];
                        print!("cd '{}'", dest_dir);
                        if dest_dir == ".." {
                            // ASSUME: If we've requested to go up one level, that level is already known.
                            //  Loop through existing dirs and find the parent
                            let cwd = get_current_working_directory(dirs.clone(), pwd);
                            println!(" - CWD {:?}", cwd);
                            pwd = cwd.parent;
                        } else if dest_dir == "/" {
                            println!(" - root");
                            pwd = dest_dir.to_string();
                        } else {
                            println!("");
                            pwd = dest_dir.to_string();
                        }
                    } else if cmd.command.contains("ls") {
                        println!("ls '{}'", pwd);
                    }
                }
                'd' => {
                    // print!("DIR");
                    let parent = pwd.clone();
                    let tmp_name = &line_string[4..];
                    let name = tmp_name.to_string();

                    // Add this dir to parent dir's list of children
                    let mut dirs2: Vec<DirFil> = Vec::new();
                    let l = dirs.len();
                    for _i in 0..l {
                        let mut tmp_dir = dirs.pop().unwrap();
                        let mut tmp_children: Vec<String> = Vec::new();
                        if tmp_dir.name == parent {
                            let blah = tmp_dir.clone().child_dirs;
                            if blah.is_none() {
                                let tmp_child: Vec<String> = vec![name.clone()];
                                tmp_children = tmp_child;
                            } else {
                                let mut tmp_child = blah.unwrap();
                                tmp_child.push(name.clone());
                                tmp_children = tmp_child;
                            }
                            tmp_dir.child_dirs = Some(tmp_children);
                        }
                        dirs2.push(tmp_dir);
                    }
                    dirs = dirs2;
                    // Add dir to vec
                    let dir = DirFil {
                        name,
                        parent,
                        child_dirs: None,
                        size: -1,
                        child_fileize: -1,
                        child_dirsize: -1,
                    };
                    // println!("Adding DIR {:?}", dir);
                    dirs.push(dir);
                    //
                }
                '0'..='9' => {
                    // print!("FIL");
                    let parent = pwd.clone();
                    let re = Regex::new(r"(\d+) (.*)").unwrap();
                    let mut size: i32 = 0;
                    let mut name: String = String::from("");
                    for cap in re.captures_iter(&line_string) {
                        size = cap.get(1).unwrap().as_str().parse().unwrap();
                        name = cap.get(2).unwrap().as_str().to_string();
                    }
                    let fil = DirFil {
                        name,
                        parent,
                        size,
                        child_dirs: None,
                        child_fileize: -1,
                        child_dirsize: -1,
                    };
                    // println!("Adding FIL {:?}", fil);
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
    let dirs2 = populate_sizes_in_dirs(dirs.clone(), fils.clone());
    dirs = dirs2;
    println!("");
}

fn get_current_working_directory(_dirs: Vec<DirFil>, _pwd: String) -> DirFil {
    let cwd = _dirs.into_iter().find(|x| x.name == _pwd).unwrap();
    cwd
}

fn populate_sizes_in_dirs(_dirs: Vec<DirFil>, _fils: Vec<DirFil>) -> Vec<DirFil> {
    let mut return_dirs: Vec<DirFil> = Vec::new();
    let dirss = _dirs.clone();
    let filss = _fils.clone();
    for mut dir in dirss.clone() {
        let mut files: Vec<DirFil> = Vec::new();
        dir.child_fileize = 0;
        for f in filss.clone() {
            if f.size >= 0 && f.parent == dir.name {
                files.push(f.clone());
            }
        }
        // println!("Children of {} {:?}", dir.name, files);
        for f in files {
            dir.child_fileize += f.size;
        }
        return_dirs.push(dir);
    }
    println!("Filsize {:?}",return_dirs);
    // add child dirs to direct parent
    // This needs to be recursive, keep going down until no children, then go back up until you can go back down again

    let t = total_size(dirss.clone());
    //
    println!("Dirsize {:?}",return_dirs);
    return_dirs
}

fn total_size(_dirfil_subset:Vec<DirFil>) -> i32 {
    let dirfil = _dirfil_subset.clone();
    let mut return_i32: i32 = 0;
    for d in dirfil.clone() {
        if d.child_dirs.is_none() {
            // no children, return
            if d.child_fileize < 0{
                // no subfiles?
                return_i32 += 0;
            } else {
                return_i32 += d.child_fileize;
            }
        } else {
            let children = d.child_dirs.unwrap();
            for c in children {
                for dd in dirfil.clone() {
                    if d.name == c {
                        return_i32 += total_size(vec![dd]);
                    }
                }
            }
        }
    }
    return_i32
}