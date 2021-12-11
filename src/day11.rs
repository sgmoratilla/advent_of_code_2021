use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::io::{BufRead, BufReader};
use ndarray::{Array2, Zip};

fn main() {
    let mut data = read_file("src/day11.txt");
    let step = first_sync(&data);
    println!("{}", step);
}

fn first_sync(energy: &Array2<u8>) -> u32 {
    let mut energy = energy.clone();

    let size = energy.nrows() * energy.ncols();
    let size = size as u32;

    let mut current_step = 0;
    let mut keep_going = true;
    while keep_going {
        let current_flashes = step(&mut energy);

        if current_flashes == size {
            keep_going = false;
        }
        current_step += 1;
    }

    return current_step;
}


fn n_flashes(energy: &Array2<u8>, steps: u32) -> u32 {
    println!("{}", energy);

    let mut energy = energy.clone();
    let mut flashes = 0;
    for i in 0..steps {
        let current_flashes = step(&mut energy);
        println!("{}", energy);

        flashes += current_flashes;
    }

    return flashes;
}

fn step(energy: &mut Array2<u8>) -> u32 {
    let mut flashes = 0;

    energy.iter_mut().for_each(|x| *x = *x + 1);

    for i in 0..energy.ncols() {
        for j in 0..energy.nrows() {
            if energy[[i, j]] > 9 {
                let new_flashes = flash(energy, i as i32, j as i32);
                flashes += new_flashes;

            }
        }
    }

    return flashes;
}

fn flash(energy: &mut Array2<u8>, i: i32, j: i32) -> u32 {
    let mut flashes = 1;
    energy[[i as usize, j as usize]] = 0;

    for x in i-1..=i+1 {
        for y in j-1..=j+1 {
            if x == i && y == j {
                continue;
            }

            if !is_valid(energy, x, y) {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            // it has just flashed, it is resting.
            if energy[(x, y)] == 0 {
                continue;
            }

            energy[(x, y)] += 1;
            if energy[(x, y)] > 9 {
                flashes += flash(energy, x as i32, y as i32);
            }
        }
    }

    return flashes;
}

fn is_valid(energy: &mut Array2<u8>, i: i32, j: i32) -> bool {
    let ui = i as usize;
    let uj = j as usize;

    if i < 0 || ui >= energy.nrows() {
        return false;
    }

    if j < 0 || uj >= energy.ncols() {
        return false;
    }

    return true;
}

fn read_file(path: &str) -> Array2<u8> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Array2<u8> {
    let lines =  reader.lines().peekable();

    let mut data = Vec::new();
    for l in lines {
        let l = l.unwrap();

        let l = l.chars().map(|x| x.to_string().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        data.push(l);
    }

    return vec_to_array2(&data);
}

fn vec_to_array2(data : &Vec<Vec<u8>>) -> Array2<u8> {
    let n =
        if data.is_empty() {
            0
        } else {
            data[0].len()
        };

    let mut arr = Array2::<u8>::default((data.len(), n));
    for i in 0..data.len() {
        for j in 0..n {
            arr[(i, j)] = (*data)[i][j];
        }
    }
    return arr;
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{first_sync, n_flashes, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data =
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let flashes = n_flashes(&data, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn day11a() {
        let data = read_file("src/day11.txt");

        let flashes = n_flashes(&data, 100);
        assert_eq!(flashes, 1739);
    }

    #[test]
    fn example2() {
        let data =
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let step = first_sync(&data);
        assert_eq!(step, 195);
    }

    #[test]
    fn day11b() {
        let data = read_file("src/day11.txt");

        let step = first_sync(&data);
        assert_eq!(step, 324);
    }

}
