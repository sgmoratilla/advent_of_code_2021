use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let fish_school = read_file("src/day6.txt");

    let min_fuel = check_all_alignments(&positions, false);
    println!("Minimum fuel: {}", min_fuel);

}

fn check_all_alignments(positions: &Vec<u32>, linear: bool) -> u32 {
    let fuel = 0;

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();


    let min_fuel =
        (*min..*max).map(|p| { align(positions, p, linear) }).min().unwrap();

    return min_fuel;
}

fn align(positions: &Vec<u32>, alignment: u32, linear: bool) -> u32 {
    let mut fuel = 0;

    for p in positions.iter () {
        fuel += cost(*p, alignment, linear);
    }

    return fuel as u32;
}

fn cost(initial: u32, end: u32, linear: bool) -> u32 {
    let min = initial.min(end);
    let max = initial.max(end);

    let interval = max - min;
    if linear {
        return interval;
    }

    return interval * (interval + 1) / 2;
}

fn read_file(path: &str) -> Vec<u32> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Vec<u32> {
    let mut lines =  reader.lines().peekable();

    let header = lines.next().unwrap().unwrap();
    let numbers = header.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    return numbers;
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{align, check_all_alignments, cost, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data = "16,1,2,0,4,2,7,1,2,14";

        let mut reader = BufReader::new(data.as_bytes());
        let positions = reader_to_data(&mut reader);

        let fuel = align(&positions, 2, true);
        assert_eq!(fuel, 37);

        let min_fuel = check_all_alignments(&positions, true);
        assert_eq!(min_fuel, 37);
    }

    #[test]
    fn day7a() {
        let positions = read_file("src/day7.txt");

        let min_fuel = check_all_alignments(&positions, true);
        assert_eq!(min_fuel, 37);
    }

    #[test]
    fn example2() {
        let data = "16,1,2,0,4,2,7,1,2,14";

        let mut reader = BufReader::new(data.as_bytes());
        let positions = reader_to_data(&mut reader);

        let fuel = cost(1, 5, false);
        assert_eq!(fuel, 10);

        let fuel = cost(5, 16, false);
        assert_eq!(fuel, 66);

        let fuel = align(&positions, 2, false);
        assert_eq!(fuel, 206);

        let fuel = align(&positions, 5, false);
        assert_eq!(fuel, 168);

        let min_fuel = check_all_alignments(&positions, false);
        assert_eq!(min_fuel, 168);
    }

    #[test]
    fn day7b() {
        let positions = read_file("src/day7.txt");

        let min_fuel = check_all_alignments(&positions, false);
        assert_eq!(min_fuel, 37);
    }
}

