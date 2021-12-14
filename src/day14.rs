use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let code = "PSVVKKCNBPNBBHNSFKBO".chars().collect();
    let rules = read_file("src/day14.txt");

    let n = evolve_fast(&code, &rules, 40);
    println!("{}", n);
}

fn evolve_fast(characters: &Vec<char>, rules: &HashMap<[char;2], char>, steps: usize) -> u64 {
    let mut counts = BTreeMap::<[char;2], u64>::new();

    for i in (1..characters.len()).rev() {
        let left = characters[i-1];
        let right = characters[i];
        counts.insert([left, right], 1);
    }

    let mut new_counts = counts.clone();

    println!("{:?}", new_counts);
    for i in 0..steps {
        new_counts = evolve_step_fast(&new_counts, rules);
        println!("{} | {:?}", i, new_counts);
    }

    let max = new_counts.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = new_counts.iter().min_by_key(|(_, v)| *v).unwrap();

    println!("{} {}", max.1, min.1);
    return max.1 - min.1;
}


fn evolve_step_fast(counts: &BTreeMap::<[char;2], u64>, rules: &HashMap<[char;2], char>) -> BTreeMap::<[char;2], u64> {
    let mut new_counts = (*counts).clone();

    for i in counts.iter() {
        let left = i.0[0];
        let right = i.0[1];
        let count = i.1;

        let new_char = *rules.get(i.0).unwrap();
        new_counts.remove(i.0);

        let old_left = *new_counts.entry([left, new_char]).or_insert(0);
        let old_right = *new_counts.entry([new_char, right]).or_insert(0);

        new_counts.insert([left, new_char], old_left + count);
        new_counts.insert([new_char, right], old_right + count);
    }

    return new_counts
}

fn evolve_slow(characters: &Vec<char>, rules: &HashMap<[char;2], char>, steps: usize) -> usize {
    let mut new_chars = (*characters).clone();
    for i in 0..steps {
        new_chars = evolve_step(&new_chars, rules);
        println!("{:?}", i);
    }


    return most_repeated(&new_chars).1 - least_repeated(&new_chars).1;
}

fn evolve_step(characters: &Vec<char>, rules: &HashMap<[char;2], char>) -> Vec<char> {
    let mut new_chars = (*characters).clone();

    for i in (1..characters.len()).rev() {
        let left = characters[i-1];
        let right = characters[i];
        let rule = [left, right];
        new_chars.insert(i, rules[&rule]);
    }

    return new_chars
}


fn most_repeated(vector: &Vec<char>) -> (char, usize) {
    let max = vector.iter()
        .fold(HashMap::<char, usize>::new(), |mut m, x| {
            *m.entry(x.clone()).or_default() += 1;
            m
        })
        .into_iter()
        .max_by_key(|(_, v)| *v);

    let max = max.unwrap();
    return max;
}

fn least_repeated(vector: &Vec<char>) -> (char, usize) {
    let min = vector.iter()
        .fold(HashMap::<char, usize>::new(), |mut m, x| {
            *m.entry(x.clone()).or_default() += 1;
            m
        })
        .into_iter()
        .min_by_key(|(_, v)| *v);

    let min = min.unwrap();
    return min;
}

fn read_file(path: &str) -> HashMap<[char;2], char> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> HashMap<[char;2], char> {
    let lines =  reader.lines().peekable();

    let mut rules = HashMap::<[char;2], char>::new();
    for l in lines {
        let l = l.unwrap();

        let parsed = sscanf::scanf!(l, "{}{} -> {}", char, char, char);
        let points = parsed.unwrap();

        rules.insert([points.0, points.1], points.2);
    }

    return rules
}



#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{evolve_slow, evolve_fast, read_file, reader_to_data};

    #[test]
    fn example1() {
        let code = "NNCB".chars().collect();
        let data =
"CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut reader = BufReader::new(data.as_bytes());
        let rules = reader_to_data(&mut reader);

        let n = evolve_slow(&code, &rules, 10);
        assert_eq!(n, 1588);
    }

    #[test]
    fn day13a() {
        let code = "PSVVKKCNBPNBBHNSFKBO".chars().collect();
        let rules = read_file("src/day14.txt");

        let n = evolve_slow(&code, &rules, 10);
        assert_eq!(n, 2584);
    }

    #[test]
    fn example2() {
        let code = "NNCB".chars().collect();
        let data =
            "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut reader = BufReader::new(data.as_bytes());
        let rules = reader_to_data(&mut reader);

        let n = evolve_fast(&code, &rules, 10);
        assert_eq!(n, 1588);
    }

    #[test]
    fn day13b() {
        let code = "PSVVKKCNBPNBBHNSFKBO".chars().collect();
        let rules = read_file("src/day14.txt");

        let n = evolve_fast(&code, &rules, 40);
        assert_eq!(n, 2584);
    }
}
