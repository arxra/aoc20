use std::fs;

fn init(contents: &Vec<&str>) -> Vec<usize> {
    let mut res = Vec::new();
    for line in contents[0..25].iter() {
        res.push(line.parse().unwrap())
    }
    res
}

fn find(contents: &Vec<&str>, last: &mut Vec<usize>, i: usize) -> usize {
    let n: usize = contents[i].parse().unwrap();
    for l in last[last.len() - i..i].iter() {
        for ll in last[last.len() - i..i].iter() {
            if l != ll && l + ll == n {
                last.push(n);
                return find(contents, last, i + 1);
            }
        }
    }
    n
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
