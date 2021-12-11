use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::io::{BufRead, BufReader};
use ndarray::{Array2, Zip};

fn main() {
    let mut data = read_file("src/day9.txt");

    println!("{}", basin_size(&mut data));

}

fn basin_size(map: &Array2<u8>) -> u32 {

    let mut basins = basins(map);

    println!("Basins\n{}", basins);

    let mut number = Vec::new();
    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            let partial = basin_size_recursive(i as i32, j as i32, &mut basins);
            if partial > 0 {
                //println!("Partial: {}", partial);
                number.push(partial);
            }
        }
    }

    number.sort();
    number.reverse();

    println!("{:?}", number);

    return number[0] * number[1] * number[2];
}

fn basin_size_recursive(i: i32, j: i32, map: &mut Array2<u8>) -> u32 {
    let ui = i as usize;
    let uj = j as usize;

    if i < 0 || i as usize >= map.nrows() {
        return 0;
    }

    if j < 0 || j as usize >= map.ncols() {
        return 0;
    }

    if map[(ui, uj)] == 0 {
        return 0;
    }

    // remove this, as it is counted.
    map[(i as usize, j as usize)] = 0;

    let value = 1 +
        basin_size_recursive(i+1, j, map) +
        basin_size_recursive(i-1, j, map) +
        basin_size_recursive(i, j+1, map) +
        basin_size_recursive(i, j-1, map);

    return value;
}


fn basins(map: &Array2<u8>) -> Array2<u8> {
    let mut basin = Array2::<u8>::default(map.dim());

    println!("{}", map);

    for ((i, j), value) in map.indexed_iter() {
        basin[[i, j]] = if lower_than_any_adjacent(i, j, map) { 1 } else { 0 };
    };

    println!("{}", basin);

    return basin;
}

fn risk_level(map: &Array2<u8>) -> u32 {
    let mut valley = Array2::<u32>::default(map.dim());

    println!("{}", map);

    for ((i, j), value) in map.indexed_iter() {
        valley[[i, j]] = if lower_than_all_adjacents(i, j, map) { *value as u32 + 1 } else { 0 };
    };

    println!("{}", valley);

    return valley.sum();
}

fn lower_than_any_adjacent(i: usize, j :usize, map: &Array2<u8>) -> bool {
    let value = map[(i, j)];

    let i = i as i8;
    let j = j as i8;

    let is_lower_than_adjacents=
        is_lower_than_adjacents(value, i, j+1 , map, false) ||
        is_lower_than_adjacents(value, i, j-1, map, false) ||
        is_lower_than_adjacents(value, i+1, j, map, false) ||
        is_lower_than_adjacents(value, i-1, j, map, false);

    return is_lower_than_adjacents;
}

fn lower_than_all_adjacents(i: usize, j :usize, map: &Array2<u8>) -> bool {
    let value = map[(i, j)];

    let i = i as i8;
    let j = j as i8;

    let is_lower_than_adjacents=
        is_lower_than_adjacents(value, i, j+1 , map, true) &&
        is_lower_than_adjacents(value, i, j-1, map, true) &&
        is_lower_than_adjacents(value, i+1, j, map, true) &&
        is_lower_than_adjacents(value, i-1, j, map, true);

    return is_lower_than_adjacents;
}

fn is_lower_than_adjacents(value: u8, i: i8, j: i8, map: &Array2<u8>, default: bool) -> bool {
    if i < 0 || i as usize >= map.nrows() {
        return default;
    }

    if j < 0 || j as usize >= map.ncols() {
        return default;
    }

    let i = i as usize;
    let j = j as usize;

    return value < map[(i, j)];
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
    use crate::{basin_size, basins, read_file, reader_to_data, risk_level};

    #[test]
    fn example1() {
        let data =
"2199943210
3987894921
9856789892
8767896789
9899965678";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let level = risk_level(&data);
        assert_eq!(level, 15);
    }

    #[test]
    fn day8a() {
        let data = read_file("src/day9.txt");

        let level = risk_level(&data);
        assert_eq!(level, 26);
    }

    #[test]
    fn example2() {
        let data =
"2199943210
3987894921
9856789892
8767896789
9899965678";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let basin = basin_size(&data);
        assert_eq!(basin, 1134);
    }

    #[test]
    fn day8b() {
        let data = read_file("src/day9.txt");

        let basin = basin_size(&data);
        assert_eq!(basin, 983725);
    }
}

