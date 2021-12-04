extern crate csv;

use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use ndarray::{Array, Array2, Axis};

fn main() {
    let (numbers, data) = read_file("src/day4.txt");

    let points = last_winner_points(&numbers, &data);

    println!("loser: {}", points);
}


fn winner_board(numbers: &Vec<u8>, boards: &Vec<Array2::<u8>>) -> u32 {

    let mut marked = Vec::<Array2::<bool>>::new();
    boards.iter().for_each(|b| marked.push(Array2::<bool>::default(b.dim())));

    for &n in numbers.iter() {
        mark(n, &mut marked, boards);
        let (won, i) = there_is_a_winner(&marked);
        if won {
            println!("index won {}, won {}", n, boards[i]);
            let winner_points = get_points(&marked[i], &boards[i]);
            return winner_points * n as u32;
        }
    }

    return 0;
}

fn last_winner_points(numbers: &Vec<u8>, boards: &Vec<Array2::<u8>>) -> u32 {
    let mut marked = Vec::<Array2::<bool>>::new();
    boards.iter().for_each(|b| marked.push(Array2::<bool>::default(b.dim())));

    let mut boards = boards.clone();

    for &n in numbers.iter() {
        mark(n, &mut marked, &boards);

        for i in (0..boards.len()).rev() {
            let b = &boards[i];
            let m  = &marked[i];

            if board_row_col_is_completed(m) {
                if boards.len() == 1 {
                    let b = &boards[0];
                    let m  = &marked[0];
                    let last_winner_points = get_points(m, b);
                    return last_winner_points * n as u32;
                }

                marked.remove(i);
                boards.remove(i);
            }
        }
    }

    return 0;
}

fn mark(n: u8, marked: &mut Vec<Array2::<bool>>, boards: &Vec<Array2::<u8>>) {

    boards.iter().zip(marked.iter_mut()).for_each(|(b, m)| {
        b.indexed_iter().for_each(|((i, j), v)| {
            if n == *v {
                (*m)[[i, j]] = true;
            }
        });
    });
}

fn there_is_a_winner(marked :&Vec<Array2::<bool>>) -> (bool, usize) {

    for (n, m) in marked.iter().enumerate() {
        if board_row_col_is_completed(m) {
            return (true, n);
        }
    }

    return (false, 0);
}

fn board_row_col_is_completed(marked :&Array2::<bool>) -> bool {

    let cols =
        marked.fold_axis(Axis(0), true, |acc, col| { *acc && *col })
              .fold(false, |acc, col| { acc || *col });
    let rows =
        marked.fold_axis(Axis(1), true, |acc, col| { *acc && *col })
              .fold( false, |acc, col| { acc || *col });

    let completed = cols || rows;

    return completed;
}

fn get_points(marked :&Array2::<bool>, board: &Array2::<u8>) -> u32 {


    return get_partial_sum(marked, board);

}

fn get_max_winner(marked :&Vec<Array2::<bool>>, boards: &Vec<Array2::<u8>>) -> (u32, usize) {

    let mut totals = Vec::new();
    for it in marked.iter().zip(boards.iter().enumerate()) {
        let (m, (i, b)) = it;

        let partial = get_partial_sum(m, b);

        totals.push((partial, i));
    }

    return totals.iter().fold((0, 0), |(max, max_index), &(x, i) | if x > max { (x, i) } else { (max, max_index ) });

}

fn get_partial_sum(marked :&Array2::<bool>, board :&Array2::<u8>) -> u32 {
    let mut unmarked_sum = 0;

    for i in 0..board.shape()[0] {
        for j in 0..board.shape()[1] {
            if !marked[(i,j)] {
                unmarked_sum += board[(i, j)] as u32;
            }
        }
    }

    return unmarked_sum;
}



fn read_file(path: &str) -> (Vec<u8>, Vec<Array2::<u8>>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> (Vec<u8>, Vec<Array2::<u8>>) {
    let mut lines =  reader.lines().peekable();

    let header = lines.next().unwrap().unwrap();
    let numbers = header.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();

    let mut boards = Vec::new();

    while lines.peek().is_some() {
        // skip first line
        lines.next();


        let mut board = Vec::new();
        for i in 0..5 {
            let line = lines.next().unwrap().unwrap();
            let line = line.trim().replace("  ", " ");
            let row = line.split(' ').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            board.push(row);
        }

        boards.push(vec_to_array2(&board));
    }

    return (numbers, boards);
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
    use std::io::{BufRead, BufReader};
    use csv::ReaderBuilder;
    use ndarray::Array2;
    use crate::{last_winner_points, read_file, reader_to_data, winner_board};
    #[test]
    fn example1() {
        let data =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";


        let mut reader = BufReader::new(data.as_bytes());
        let (numbers, data) = reader_to_data(&mut reader);

        let points = winner_board(&numbers, &data);
        assert_eq!(points, 4512);
    }

    #[test]
    fn day4a() {
        let (numbers, data) = read_file("src/day4.txt");

        let points = winner_board(&numbers, &data);
        assert_eq!(points, 46332);
    }

    #[test]
    fn example2() {
        let data =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";


        let mut reader = BufReader::new(data.as_bytes());
        let (numbers, data) = reader_to_data(&mut reader);

        let points = last_winner_points(&numbers, &data);
        assert_eq!(points, 1924);
    }

    #[test]
    fn day4b() {
        let (numbers, data) = read_file("src/day4.txt");

        let points = last_winner_points(&numbers, &data);
        assert_eq!(points, 21070);
    }

}

