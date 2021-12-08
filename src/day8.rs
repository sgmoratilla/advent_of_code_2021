use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::io::{BufRead, BufReader};

fn main() {
    let data = read_file("src/day8.txt");

    let count = count_all(&data);
    println!("{}", 1041746);

}

fn count_all(lines: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
    let mut count : i32 = 0;

    for l in lines {
        let encoding = feed_encoding(&l.0);

        let mut number : i32 = 0;
        for digits in (*l).1.iter() {
            let d = detect_digit(&encoding, digits);

            println!("Digit: {}", d);

            if d == -1 {
                println!("Digits not decoded, aborting");
                process::abort();
            }

            number = number * 10;
            number += d as i32;
        }

        println!("Detected: {}", number);

        count += number;
    }

    return count;
}

fn feed_encoding(examples: &Vec<String>) -> [String; 10] {
    let one_encoding = examples.iter().find(|x| x.len() == 2).unwrap();
    let four_encoding = examples.iter().find(|x| x.len() == 4).unwrap();
    let seven_encoding = examples.iter().find(|x| x.len() == 3).unwrap();
    let eight_encoding = String::from("abcdefg");

    println!("1: {}, 4: {}, 7: {}", one_encoding, four_encoding, seven_encoding);

    let encoding = [String::new(), one_encoding.clone(), String::new(), String::new(), four_encoding.clone(), String::new(), String::new(), seven_encoding.clone(), eight_encoding, String::new()];
    return encoding;
}

fn contains(data: &String, contains: &String) -> bool {
    for i in contains.chars() {
        if data.contains(i) == false {
            return false;
        }
    }

    return true;
}

fn subtract(data1: &String, data2: &String) -> String {
    let mut distinct = data1.clone();

    distinct.retain(|x| data2.contains(x) == false);

    return distinct;
}

fn matches(s1: &String, s2: &String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            return false;
        }
    }
    return true;
}

fn detect_digit(encoding: &[String; 10], data: &String) -> i8 {
    let len = data.len();
    return
        match len {
            2 => {
                // i could assert here that data is encoding[1]
                1
            },
            3 => {
                // i could assert here that data is encoding[7]
                7
            },
            4 => {
                // i could assert here that data is encoding[4]
                4
            },
            5 => {
                let diff = subtract(&encoding[8], &encoding[4]);
                // if contains all chars from 1, then is 3
                // else if contains all chars not shared by 4 and 8 then 2
                // else 5
                if contains(data, &encoding[1]) { 3 }
                else if contains(data, &diff) { 2 }
                else { 5 }
            },
            6 => {
                // if contains all chars from 4, then is 9
                // else if contains all chars from 7, then is 0
                // else 6
                if contains(data, &encoding[4]) { 9 }
                else if contains(data, &encoding[7]) { 0 }
                else { 6 }
            },
            7 => 8,
            _ => -1,
        }
}

fn count_1_4_7_8(lines: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut count = 0;

    for l in lines {
        for digits in (*l).1.iter() {
            let size = digits.len();
            if [2, 3, 4, 7].contains(&size) {
                count += 1;
            }
        }
    }

    return count;

}

fn read_file(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Vec<(Vec<String>, Vec<String>)> {
    let lines =  reader.lines().peekable();

    let mut data = Vec::new();
    for l in lines {
        let l = l.unwrap();
        let parts = l.trim().split('|').map(|x| String::from(x)).collect::<Vec<String>>();

        let first = parts.get(0).unwrap().trim();
        let second = parts.get(1).unwrap().trim();

        let first = first.split(' ').map(|x| String::from(x)).collect::<Vec<String>>();
        let second = second.split(' ').map(|x| String::from(x)).collect::<Vec<String>>();

        data.push((first, second));
    }

    return data;
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{count_1_4_7_8, count_all, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data =
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let count = count_1_4_7_8(&data);
        assert_eq!(count, 26);
    }

    #[test]
    fn day8a() {
        let data = read_file("src/day8.txt");

        let count = count_1_4_7_8(&data);
        assert_eq!(count, 26);
    }

    #[test]
    fn example2() {
        let data =
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let count = count_all(&data);
        assert_eq!(count, 61229);
    }

    #[test]
    fn day8b() {
        let data = read_file("src/day8.txt");

        let count = count_all(&data);
        assert_eq!(count, 1041746);
    }
}

