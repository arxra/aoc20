use itertools::Itertools;
use std::fs; // 0.8.0

fn valid(contents: &str, eye_colors: &[&str], hcl: &[char]) -> usize {
    let contents: String = contents.split_whitespace().intersperse(" ").collect();
    print!("contents: {}", contents);
    let mut i = 0;
    let mut check = [false; 7];
    for field in contents.split_whitespace() {
        i += 1;
        match &field[0..3] {
            "byr" => {
                check[0] = true;
                let year = &field[4..8].parse::<usize>().unwrap_or_default();
                if 1920 <= *year && *year <= 2002 {
                    continue;
                }
                return 0;
            }
            "ecl" => {
                check[1] = true;
                if field.len() == 7 && eye_colors.contains(&&field[4..7]) {
                    continue;
                }
                return 0;
            }
            "pid" => {
                check[2] = true;
                if field.len() != 13 {
                    return 0;
                }
                let pid = &field[4..13].parse::<usize>().unwrap_or_default();
                if *pid != 0 {
                    continue;
                }
                return 0;
            }
            "hcl" => {
                check[3] = true;
                if field.len() != 11 {
                    println!("-hcl, not right len");
                    return 0;
                }
                if field.chars().nth(4).unwrap() != '#' {
                    println!("-hcl, no #");
                    return 0;
                }
                for c in field[5..11].chars() {
                    if !hcl.contains(&c) {
                        return 0;
                    }
                }
                continue;
            }
            "eyr" => {
                check[4] = true;
                let year = &field[4..8].parse::<usize>().unwrap_or_default();
                if 2020 <= *year && *year <= 2030 {
                    continue;
                }
                println!("eyr: year {} not in 2020..=2030", year);
                return 0;
            }
            "iyr" => {
                check[5] = true;
                let year = &field[4..8].parse::<usize>().unwrap_or_default();
                if 2010 <= *year && *year < 2021 {
                    continue;
                }
                println!("iyr: year {} not in 2010..=2020", year);
                return 0;
            }
            "hgt" => {
                check[6] = true;
                // this can be in cm or made up eagle units
                if field.contains("cm") {
                    let year = &field[4..7].parse::<usize>().unwrap_or_default();
                    if 150 <= *year && *year <= 193 {
                        continue;
                    }
                    println!("hgt: height {} not in 150..=193cm or 59..=76in", year);
                } else if field.contains("in") {
                    let year = &field[4..6].parse::<usize>().unwrap_or_default();
                    if 59 <= *year && *year <= 76 {
                        continue;
                    }
                    println!("hgt: height {} not in 150..=193cm or 59..=76in", year);
                }
                return 0;
            }
            "cid" => {
                // This should be ignored on purpose
                continue;
            }
            a => {
                unimplemented!("all fields should be matched! {} was not ", a);
            }
        }
    }
    if i < 7 || i > 8 {
        println!("not enough or to many args: {}", i);
        return 0;
    }
    for b in check.iter() {
        if !b {
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
    let mut hcl: Vec<char> = ('0'..='9').collect();
    for c in 'a'..='f' {
        hcl.push(c);
    }
    let re = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for pass in contents.split("\n\n") {
        let v = valid(pass, &re, &hcl);
        println!(": {}", v);
        res += v;
    }

    println!("res: {}", res);
}
