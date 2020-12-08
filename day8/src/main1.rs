use std::fs;

fn parse_statement(
    contents: Vec<&str>,
    visited: &mut Vec<usize>,
    line: usize,
    acc: isize,
) -> isize {
    if visited.contains(&line) {
        return acc;
    }
    visited.push(line);
    println!("handling line {}: {}", line, &contents[line]);
    match &contents[line][0..3] {
        "acc" => match &contents[line].chars().nth(4).unwrap() {
            '+' => {
                let val = &contents[line][5..].parse::<isize>().unwrap();
                parse_statement(contents, visited, line + 1, acc + val)
            }
            '-' => {
                let val = &contents[line][5..].parse::<isize>().unwrap();
                parse_statement(contents, visited, line + 1, acc - val)
            }
            other => {
                panic!("Other token encountered, not supported: {}", other)
            }
        },
        "nop" => parse_statement(contents, visited, line + 1, acc),
        "jmp" => match &contents[line].chars().nth(4).unwrap() {
            '+' => {
                let line = line + contents[line][5..].parse::<usize>().unwrap();
                parse_statement(contents, visited, line, acc)
            }
            '-' => {
                let line = line - contents[line][5..].parse::<usize>().unwrap();
                parse_statement(contents, visited, line, acc)
            }
            other => {
                panic!("Other token encountered, not supported: {}", other)
            }
        },
        other => {
            panic!("Other token encountered, not supported: {}", other)
        }
    }
}

fn main() {
    // --snip--
    let filename = "input.txt";
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<&str> = contents.lines().collect();
    let res = parse_statement(contents, &mut Vec::new(), 0, 0);
    println!("res: {}", res);
}
