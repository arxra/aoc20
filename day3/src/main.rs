use std::fs;

fn slope(contents: Vec<&str>, r: usize, d: usize) -> usize {
    println!("calculating slope: {}:{}", r, d);
    let mut col: usize = 0;
    let mut row: usize = 0;
    let mut res: usize = 0;
    let cmax = 31;

    while row <= 322 {
        if contents[row].chars().nth(col).unwrap() == '#' {
            res += 1;
        }
        col = (col + r).rem_euclid(cmax);
        row += d;
    }
    res
}
fn main() {
    // --snip--
    let filename = "input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<&str> = contents.lines().collect();
    let indatr: [usize; 5] = [1, 3, 5, 7, 1];
    let indatd: [usize; 5] = [1, 1, 1, 1, 2];
    let mut res = 1;
    for (r, d) in indatr.iter().zip(indatd.iter()) {
        res *= slope(contents.clone(), *r, *d);
    }
    println!("res: {}", res);
}
