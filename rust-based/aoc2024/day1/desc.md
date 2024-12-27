For example:
```log
3   4
4   3
2   5
1   3
3   9
3   3
```
Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.

Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.

PsuedoCode

Read line
Add first number to first list (array, Vector)
add second number to second list
sort each list
Loop through first list with index/value
    Get distance (difference (abs(x-y))) between
    Add distance to "total"
Output total

Alternate
Find minimum value in each vector, remove them.
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=276212e20fb404f3d8c23153a185f31a
```rust
fn main() {
    let vec_to_check = vec![5, 6, 8, 4, 2, 7];
    let min_value = vec_to_check.iter().min();
    match min_value {
        None => println!("Min value was not found"),
        Some(i) => println!("Min Value = {}", i)
    }
}
```

Question: Are there duplicate values in either vector?
https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.counts
