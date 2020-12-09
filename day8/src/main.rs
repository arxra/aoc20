use std::fs;

fn parse_linechange(contents: &Vec<&str>, line: usize) -> usize {
    match &contents[line].chars().nth(4).unwrap() {
        '+' => line + contents[line][5..].parse::<usize>().unwrap(),
        '-' => line - contents[line][5..].parse::<usize>().unwrap(),
        other => {
            panic!("Other token encountered, not supported: {}", other)
        }
    }
}

fn parse_statement(
    contents: &Vec<&str>,
    visited: &mut Vec<usize>,
    line: usize,
    acc: isize,
) -> Option<isize> {
    if visited.contains(&line) {
        return None;
    } else if line >= 622 {
        println!("line {} reached!", line);
        return Some(acc);
    }
    visited.push(line);
    println!("handling line {}: {}", line, &contents[line]);
    match &contents[line][0..3] {
        "acc" => {
            let val = &contents[line][4..].parse::<isize>().unwrap();
            parse_statement(contents, visited, line + 1, acc + val)
        }
        "nop" => match parse_statement(&contents, visited, line + 1, acc) {
            None => parse_statement(&contents, visited, parse_linechange(contents, line), acc),
            Some(acc) => Some(acc),
        },
        "jmp" => match parse_statement(&contents, visited, parse_linechange(contents, line), acc) {
            None => parse_statement(contents, visited, line + 1, acc),
            Some(acc) => Some(acc),
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
    let res = parse_statement(&contents, &mut Vec::new(), 0, 0);
    println!("res: {}", res.unwrap());
}
