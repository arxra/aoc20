use std::fs;

fn main() {
    // --snip--
    let filename = "input2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut res: usize = 0;
    for l in contents.lines() {
        // 16-18 h: hhhhhhhhhhhhhhhhhh as example
        let mut splits = l.split_whitespace();
        let n = splits.next().unwrap();
        let var = splits
            .next()
            .unwrap()
            .trim_right_matches(":")
            .chars()
            .next()
            .unwrap();
        println!("found char: {} from {}", var, l);
        let mut chars = 0;
        for c in splits.next().unwrap().chars() {
            if c == var {
                chars += 1;
            }
        }
        let min = n.split_at(n.find("-").unwrap());
        let mi = min.0.parse::<usize>().unwrap();
        let ma = min.1.trim_left_matches("-").parse::<usize>().unwrap();
        if chars >= mi && chars <= ma {
            res += 1;
        }

        println!("{}: {} - {}", chars, mi, ma + mi);
    }
    println!("Res: {}", res);
}
