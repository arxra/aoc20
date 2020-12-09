use std::{cmp, fs};

use cmp::Ordering::{Equal, Less};

fn init(contents: &Vec<&str>) -> Vec<usize> {
    let mut res = Vec::new();
    for line in contents[0..25].iter() {
        res.push(line.parse().unwrap())
    }
    res
}

fn find_sum(last: &mut Vec<usize>, low: usize, high: usize, t: usize) -> (usize, usize) {
    let mut acc = 0;
    let mut h = low;
    while h != high {
        acc += last[h];
        h += 1;
        match t.cmp(&acc) {
            Equal => {
                return (
                    *last[low..=h].iter().min().unwrap(),
                    *last[low..=h].iter().max().unwrap(),
                )
            }
            Less => {
                break;
            }
            _ => {}
        }
    }
    find_sum(last, low + 1, high, t)
}

fn find(contents: &Vec<&str>, last: &mut Vec<usize>, i: usize) -> usize {
    let n: usize = contents[i].parse().unwrap();
    let mut line = 0;
    for l in last[last.len() - i..i].iter() {
        for ll in last[last.len() - i..i].iter() {
            if l != ll && l + ll == n {
                last.push(n);
                return find(contents, last, i + 1);
            }
        }
        line += 1;
    }
    println!("line for num: {}", line);
    let (r1, r2) = find_sum(last, 0, line, n);
    r1 + r2
}

fn main() {
    // --snip--
    let filename = "input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<&str> = contents.lines().collect();
    let mut last = init(&contents);
    let res = find(&contents, &mut last, 25);
    println!("res: {}", res);
}
