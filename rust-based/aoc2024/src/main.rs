pub mod libz;
use std::convert::TryFrom;

fn main() {
    // println!("Hello World!");
    // println!("{}", libz::add(9,10));
    // libz::testday1();

    // let i = libz::get_input(1, true);
    // println!("{}", libz::vec2string(i));

    println!("{}", day1(libz::get_input(1, false)));
}

fn day1(data: Vec<u8>) -> String {
    // Null Helper to initialze and clear Vectors
    let nhu = &[0u8];
    let nhi = &[0i32];
    let mut left: Vec<i32> = nhi.to_vec();
    let mut right: Vec<i32> = nhi.to_vec();
    let mut lbuff: Vec<u8> = nhu.to_vec();
    let mut rbuff: Vec<u8> = nhu.to_vec();
    let mut buff: Vec<u8> = nhu.to_vec();
    left.clear();
    right.clear();
    rbuff.clear();
    lbuff.clear();
    buff.clear();
    // Read line
    //Add first number to first list (array/Vector)
    //add second number to second list
    //sort each list

    // HINT:
    // SPACE   == 32u8
    // NEWLINE == 10u8
    for c in data.iter() {
        // println!("Char:'{}'",c);
        if *c == 10u8 {
            // print!("NEWLINE:'{}'", c);
            // print!(" Buffer length:'{}'", buff.len());
            let s = libz::vec2string(buff.clone());
            // print!(" Buffer contents:'{}'",s);
            // println!();
            let strings: Vec<&str> = s.split_whitespace().collect();
            match strings[0].parse::<i32>() {
                Ok(n) => left.push(n),
                Err(e) => println!("Error: {}", e),
            };
            match strings[1].parse::<i32>() {
                Ok(n) => right.push(n),
                Err(e) => println!("Error: {}", e),
            };
            buff.clear();
        } else {
            buff.push(*c);
        }

    }

    let d1p1 = day1part1(left.clone(), right.clone());
    let d1p2 = day1part2(left.clone(), right.clone());
    return format!("{} {}",d1p1,d1p2)

}

fn day1part1(l: Vec<i32>, r: Vec<i32>) -> u32 {
    let mut left = l.clone();
    let mut right = r.clone();
    let mut total: u32 = 0;
    
    left.sort();
    right.sort();

    let itemcount: usize = left.len();
    for idx in 0..itemcount {
        // print!("'{:?}-{:?}'",left[idx],right[idx]);
        let v = left[idx].abs_diff(right[idx]);
        // println!("={:?}",v);
        total += v;
    }
    // println!("{}",itemcount);
    return total;
}

fn day1part2(l: Vec<i32>, r: Vec<i32>) -> u32 {
    let mut total: u32 = 0;
    for i in l {
        let c = day1counter(r.clone(), i);
        // Multiply count by int. If count is 0, then score is 0.
        let score = c * u32::try_from(i).unwrap();
        total += score;
    }
    return total;
}

fn day1counter(v: Vec<i32>, int_to_count: i32) -> u32 {
    let mut v2 = v.clone();
    // Only keep elements if they're the same as int_to_count
    v2.retain(|&x| x == int_to_count);
    // Count of remaining elements
    let c = u32::try_from(v2.len()).unwrap();
    return c
}

