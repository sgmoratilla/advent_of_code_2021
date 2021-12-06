use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let fish_school = read_file("src/day6.txt");

}

fn evolve(fish_school: &Vec<u8>, generations: u32) -> u64 {
    let mut offspring = fish_school.clone();

    for _i in 0..generations {
        let mut new_fish = 0;
        for f in offspring.iter_mut() {
            if *f == 0 {
                *f = 8;
                new_fish = new_fish + 1;
            } else {
                *f = *f - 1;
            }
        }

        (0..new_fish).for_each(|_| offspring.push(6));
    }

    return offspring.len() as u64;
}

fn evolve_fast(fish_school: &Vec<u8>, generations: u32) -> u64 {

    let mut n_fishes : u64 =  0;
    for f in fish_school.iter() {
        n_fishes += count_offspring(*f as i32, generations as i32);
    }

    return n_fishes;
}

fn count_offspring(time_to_reproduce: i32, generations: i32) -> u64 {
    let reproduces = time_to_reproduce < generations;
    if reproduces {
        return
            count_offspring(6, generations - 6) +
            count_offspring(8, generations - 8);
    }

    return 1;
}


fn read_file(path: &str) -> Vec<u8> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Vec<u8> {
    let mut lines =  reader.lines().peekable();

    let header = lines.next().unwrap().unwrap();
    let numbers = header.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();

    return numbers;
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{evolve, evolve_fast, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data = "3,4,3,1,2";

        let mut reader = BufReader::new(data.as_bytes());
        let numbers = reader_to_data(&mut reader);

        let fish_number = evolve(&numbers, 18);
        assert_eq!(fish_number, 26);

        let fish_number = evolve(&numbers, 80);
        assert_eq!(fish_number, 5934);
    }

    #[test]
    fn day5a() {
        let numbers = read_file("src/day6.txt");

        let fish_number = evolve(&numbers, 80);

        assert_eq!(fish_number, 350917);
    }

    #[test]
    fn test_fast_evolve() {
        let data = "6";
        /* generation  fish
                       6
                    1  5
                    2  4
                    3  3
                    4  2
                    5  1
                    6  0
                    7  6 8
                    8  5 7
                    9  4 6
                    10 3 5
                    11 2 4
                    12 1 3
                    13 0 2
                    14 6 1 8
                    15 5 0 7
                    16 4 6 6 8
         */

        let mut reader = BufReader::new(data.as_bytes());
        let numbers = reader_to_data(&mut reader);

        let fish_number = evolve_fast(&numbers, 6);
        assert_eq!(fish_number, 1);

        let fish_number = evolve_fast(&numbers, 7);
        assert_eq!(fish_number, 2);

        let fish_number = evolve_fast(&numbers, 12);
        assert_eq!(fish_number, 2);

        let fish_number = evolve_fast(&numbers, 13);
        assert_eq!(fish_number, 2);

        let fish_number = evolve_fast(&numbers, 14);
        assert_eq!(fish_number, 3);

        let fish_number = evolve_fast(&numbers, 15);
        assert_eq!(fish_number, 3);

        let fish_number = evolve_fast(&numbers, 16);
        assert_eq!(fish_number, 3+1);

        return;

        //let fish_number = evolve_fast(&numbers, 256);
        //assert_eq!(fish_number, 2);

        let data = "6,3";

        let mut reader = BufReader::new(data.as_bytes());
        let numbers = reader_to_data(&mut reader);
        let fish_number = evolve_fast(&numbers, 6);
        assert_eq!(fish_number, 1+2);

        let fish_number = evolve_fast(&numbers, 7);
        assert_eq!(fish_number, 2+2);

        let fish_number = evolve_fast(&numbers, 10);
        assert_eq!(fish_number, 2+2);
    }

    #[test]
    fn example2() {
        let data = "3,4,3,1,2";

        let mut reader = BufReader::new(data.as_bytes());
        let numbers = reader_to_data(&mut reader);

        let fish_number = evolve_fast(&numbers, 18);
        assert_eq!(fish_number, 26);

        let fish_number = evolve_fast(&numbers, 80);
        assert_eq!(fish_number, 5934);

    }

    #[test]
    fn day5b() {
        let numbers = read_file("src/day6.txt");

        let fish_number = evolve(&numbers, 256);

        assert_eq!(fish_number, 350917);
    }
}

