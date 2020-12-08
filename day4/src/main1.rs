use std::fs;

fn valid(contents: &str, re: &[&str]) -> usize {
    for r in re.iter() {
        if !contents.contains(r) {
            return 0;
        }
    }
    1
}
fn main() {
    let filename = "input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut res: usize = 0;
    let re = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

    for pass in contents.split("\n\n") {
        res += valid(pass, &re);
    }

    println!("res: {}", res);
}
