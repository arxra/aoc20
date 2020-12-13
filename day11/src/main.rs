use std::fs;

fn check_seat(map: &[Vec<char>], r: usize, c: usize) -> usize {
    if map[r][c] == '#' {
        1
    } else {
        0
    }
}

fn sum_adjencent(map: Vec<Vec<char>>, r: usize, c: usize) -> usize {
    let mut count = 0;
    if r != 0 && c != 0 {
        count += check_seat(&map, r - 1, c - 1);
    }
    if r != 0 && c + 1 < map[r].len() {
        count += check_seat(&map, r - 1, c + 1);
    }
    if r != 0 {
        count += check_seat(&map, r - 1, c);
    }
    if c != 0 && r + 1 < map.len() {
        count += check_seat(&map, r + 1, c - 1);
    }
    if c != 0 {
        count += check_seat(&map, r, c - 1);
    }
    if r < map.len() - 1 {
        count += check_seat(&map, r + 1, c);
    }
    if c + 1 < map[r].len() {
        count += check_seat(&map, r, c + 1);
    }
    if c < map[r].len() - 1 && r < map.len() - 1 {
        count += check_seat(&map, r + 1, c + 1);
    }
    count
}

fn seat(map: &mut Vec<Vec<char>>) -> &Vec<Vec<char>> {
    let mut temp: Vec<(usize, usize)> = Vec::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '.' {
                continue;
            }
            let s = sum_adjencent(map.clone(), r, c);
            if (s >= 4 && map[r][c] == '#') || (s == 0 && map[r][c] == 'L') {
                temp.push((r, c));
            }
        }
    }
    if temp.is_empty() {
        map
    } else {
        println!("map not stable: {} changes", temp.len(),);
        for (r, c) in temp {
            if map[r][c] == '#' {
                map[r][c] = 'L';
            } else if map[r][c] == 'L' {
                map[r][c] = '#';
            } else {
                println!("Trying to change {} ?? ", map[r][c])
            }
        }
        seat(map)
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut map: Vec<Vec<char>> = Vec::new();
    for (line, l) in contents.lines().enumerate() {
        map.push(Vec::new());
        for c in l.chars() {
            map[line].push(c)
        }
    }

    let mut count = 0;
    seat(&mut map)
        .iter()
        .for_each(|inner| count += inner.iter().filter(|&n| *n == '#').count());
    println!("res: {:?}", count);
}
