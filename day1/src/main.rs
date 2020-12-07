use std::fs;
use std::{env, println};

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut ans: Vec<i32> = Vec::new();
    for l in contents.lines() {
        ans.push(l.parse::<i32>().unwrap())
    }
    for i in ans.clone() {
        for ii in ans.clone() {
            for iii in ans.clone() {
                if i + ii + iii == 2020 {
                    println!("{}", i * ii * iii);
                    return;
                }
            }
        }
    }
}
