use std::fs;

fn main() {
    // --snip--
    let filename = "input2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut res: usize = 0;
    let mut chars;
    for l in contents.lines() {
        // 16-18 h: hhhhhhhhhhhhhhhhhh as example
        chars = 0;
        let mut splits = l.split_whitespace();
        let n = splits.next().unwrap();
        let var = splits
            .next()
            .unwrap()
            .trim_end_matches(':')
            .chars()
            .next()
            .unwrap();
        println!("found char: {} from {}", var, l);
        let min = n.split_at(n.find('-').unwrap());
        let mi = min.0.parse::<usize>().unwrap();
        let ma = min.1.trim_start_matches('-').parse::<usize>().unwrap();
        if splits.clone().next().unwrap().chars().nth(mi - 1).unwrap() == var {
            chars += 1;
        }
        if splits.next().unwrap().chars().nth(ma - 1).unwrap() == var {
            chars += 1;
        }
        if chars == 1 {
            res += 1;
        }
    }
    println!("Res: {}", res);
}
