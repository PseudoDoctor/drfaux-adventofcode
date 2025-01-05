pub mod libz;

fn main() {
    // println!("Hello World!");
    // println!("{}", libz::add(9,10));
    // libz::testday1();

    // let i = libz::get_input(1, true);
    // println!("{}", libz::vec2string(i));

    println!("{}", day1(libz::get_input(1, true)));
}

fn day1(data: Vec<u8>) -> i32 {
    // Null Helper to initialze and clear Vectors
    let nhu = &[0u8];
    let nhi = &[0i32];
    let mut _left: Vec<i32> = nhi.to_vec();
    let mut _right: Vec<i32> = nhi.to_vec();
    let mut total: i32 = 0;
    let mut lbuff: Vec<u8> = nhu.to_vec();
    let mut rbuff: Vec<u8> = nhu.to_vec();
    let mut buff: Vec<u8> = nhu.to_vec();
    _left.clear();
    _right.clear();
    rbuff.clear();
    lbuff.clear();
    buff.clear();
    // Read line
    //Add first number to first list (array/Vector)
    //add second number to second list
    //sort each list
    //Loop through first list with index/value
    //Get distance (difference (abs(x-y))) between
    //Add distance to "total"
    //Output total

    // HINT:
    // SPACE   == 32u8
    // NEWLINE == 10u8
    for c in data.iter() {
        // println!("Char:'{}'",c);
        if *c == 10u8 {
            print!("NEWLINE:'{}'", c);
            print!(" Buffer length:'{}'", buff.len());
            // process rbuff
            if rbuff.len() > 0 {
                let s = libz::vec2string((*rbuff).to_vec());
                print!("Left String: '{}'", s);
                match s.parse::<i32>() {
                    Ok(n) => _right.push(n),
                    Err(e) => println!("Error: {}", e),
                }
            }
            // clear buffers
            rbuff.clear();
            buff.clear();
        } else if *c == 32u8 {
            print!("SPACE:  '{}'", c);
            print!(" Buffer length:'{}'", buff.len());
            buff.push(*c);
            // if lbuff has stuff, process
            if lbuff.len() > 0 {
                let s = libz::vec2string((*lbuff).to_vec());
                print!("Left String: '{}'", s);
                match s.parse::<i32>() {
                    Ok(n) => _left.push(n),
                    Err(e) => println!("Error: {}", e),
                }
            }
            lbuff.clear();
        } else {
            print!("CHAR:   '{}'", c);
            // Decide which buffer to add char to

            print!(" Buffer length:'{}'", buff.len());
            if buff.len() == 0 {
                // If buff is empty, add to lbuff
                lbuff.push(*c);
            } else {
                // If buff is not empty, look at last element
                let l = buff.last().copied().unwrap();
                print!(" LastBuff: {}", l);
                if l != 32u8 {
                    // If last element is not SPACE, add to lbuff
                    lbuff.push(*c);
                } else {
                    // Else add to rbuff
                    rbuff.push(*c);
                }
            }
            //Always add to line buffer
            buff.push(*c);
        }
        println!();
    }

    // sort vectors
    println!("{:?}",_left);
    println!("{:?}",_right);
    _left.sort();
    _right.sort();
    println!("{:?}",_left);
    println!("{:?}",_right);


    // total += 1;
    return total;
}
