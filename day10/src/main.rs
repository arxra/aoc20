use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn parse_adapter(contents: &[usize], mem: &mut HashMap<usize, usize>) -> usize {
    if mem.contains_key(contents.first().unwrap()) {
        // if we know this lovation, then just return its value
        *mem.get(contents.first().unwrap()).unwrap()
    } else if contents.len() == 1 {
        // if we are the last value, just return it
        mem.insert(*contents.first().unwrap(), 1);
        1
    } else {
        // Here we must calculate the posibilites we got where we are
        let mut c = 0;
        // Skip the first item, as that's us. Otherwise we would inf recurse here
        for (i, n) in contents[1..].iter().enumerate() {
            if n - contents.first().unwrap() <= 3 {
                c += parse_adapter(&contents[i + 1..], mem);
            } else {
                break;
            }
        }
        mem.insert(*contents.first().unwrap(), c);
        c
    }
}

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut contents: Vec<usize> = contents
        .split_whitespace()
        .map(|c| usize::from_str(c).unwrap())
        .collect();
    contents.push(0); // Don't forget the start
    contents.sort_unstable();
    contents.push(contents.last().unwrap() + 3); // Or forget the end nodes
    let mut mem = HashMap::new();
    let count = parse_adapter(&contents, &mut mem);
    println!("There are {} ways to compibne them", count);
}
