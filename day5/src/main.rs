use std::fs;

fn parse_id(seat: str) -> usize {
    let mut i = 0;
    let mut r = 0;
}

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
}
