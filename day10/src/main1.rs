use std::fs;
use std::str::FromStr;

fn parse_adapter(contents: Vec<usize>, res: (usize, usize), last: usize) -> (usize, usize) {
    if contents.contains(&(last + 1)) {
        parse_adapter(contents, (res.0 + 1, res.1), last + 1)
    } else if contents.contains(&(last + 2)) {
        parse_adapter(contents, res, last + 2)
    } else if contents.contains(&(last + 3)) {
        parse_adapter(contents, (res.0, res.1 + 1), last + 3)
    } else {
        println!("done, no more after {}", last);
        res
    }
}

fn main() {
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<usize> = contents
        .split_whitespace()
        .map(|c| usize::from_str(c).unwrap())
        .collect();
    let (one, three) = parse_adapter(contents, (0, 1), 0);
    println!("mul {}*{} = {}", one, three, one * three);
}
