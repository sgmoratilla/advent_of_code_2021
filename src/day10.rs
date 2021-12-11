use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::io::{BufRead, BufReader};
use ndarray::{Array2, Zip};

fn main() {
    let mut data = read_file("src/day10.txt");

}

fn find_completion_points(lines: &Vec<String>) -> u64 {
    let mut points = Vec::new();

    for l in lines {
        println!("{}", l);
        let mut stack = find_valid_stacks(l);
        stack.reverse();
        let mut stack = stack.iter().map(|s| closing(*s)).collect::<Vec<char>>();
        if !stack.is_empty() {
            let p = stack_points(&stack);
            points.push(p);
        }
    }

    points.sort();

    let middle = points.len() / 2;

    return points[middle];
}

fn stack_points(stack: &Vec<char>) -> u64 {
    let mut p = 0 ;

    for c in stack {
        p = p * 5;
        p = p + fixing_points(*c) as u64;
    }

    return p;
}

fn fixing_points(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => process::abort()
    }
}


fn find_valid_stacks(line: &String) -> Vec<char> {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let last = stack.pop();
            if last.is_none() || closing(last.unwrap()) != c {
                return Vec::new();
            }
        }
    }

    return stack;
}

fn points(lines: &Vec<String>) -> u32 {

    let points =
        find_illegal_chars(lines).iter().map(|&x| points_char(x)).sum();

    return points;
}

fn points_char(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => process::abort()
    }
}

fn find_illegal_chars(lines: &Vec<String>) -> Vec<char> {
    let mut invalid = Vec::new();
    for l in lines {
        println!("{}", l);
        let first = find_first_illegal_char(l);
        if first.is_some() {
            invalid.push(first.unwrap());
        }
    }

    return invalid;
}

fn find_first_illegal_char(line: &String) -> Option<char> {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let last = stack.pop();
            if last.is_none() || closing(last.unwrap()) != c {
                return Some(c);
            }
         }
    }

    return None;
}

fn is_opening(c: char) -> bool {
    return ['{', '[', '(', '<'].contains(&c);
}

fn opening(c: char) -> char {
    match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        '>' => '<',
        _ => process::abort()
    }
}

fn closing(c: char) -> char {
    match c {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        '<' => '>',
        _ => process::abort()
    }
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Vec<String> {
    let lines =  reader.lines().peekable();

    let mut data = Vec::new();
    for l in lines {
        let l = l.unwrap();

        data.push(l);
    }

    return data;
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{find_completion_points, points, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data =
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let points = points(&data);
        assert_eq!(points, 26397);
    }

    #[test]
    fn day10a() {
        let data = read_file("src/day10.txt");

        let points = points(&data);
        assert_eq!(points, 436497);
    }

    #[test]
    fn example2() {
        let data =
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let points = find_completion_points(&data);
        assert_eq!(points, 288957);
    }

    #[test]
    fn day10b() {
        let data = read_file("src/day10.txt");

        let points = find_completion_points(&data);
        assert_eq!(points, 288957);
    }
}
