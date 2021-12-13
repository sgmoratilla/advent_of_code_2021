use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::env::current_exe;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::Map;
use ndarray::Array2;

enum Axis {
    X,
    Y
}


fn main() {
    let mut data = read_file("src/day13.txt");

    let instructions = vec![
        (Axis::X, 655),
        (Axis::Y, 447),
        (Axis::X, 327),
        (Axis::Y, 223),
        (Axis::X, 163),
        (Axis::Y, 111),
        (Axis::X, 81),
        (Axis::Y, 55),
        (Axis::X, 40),
        (Axis::Y, 27),
        (Axis::Y, 13),
        (Axis::Y, 6),
    ];
    fold(&data, &instructions);

    // Remove the 0, from the print, and enjoy the code CEJKLUGJ
    println!("{:?}", new_paper);
}

fn fold(paper: &Array2<u32>, instructions: &Vec<(Axis, usize)>) -> u32 {


    let mut new_paper = (*paper).clone();
    for i in instructions {
        new_paper = fold_one(&new_paper, i);

    }

    return new_paper.sum();
}

fn fold_one(paper: &Array2<u32>, instructions: &(Axis, usize)) -> Array2<u32>  {
    let mut paper = (*paper).clone();

    match instructions.0 {
        Axis::Y => {
            let fold_y = instructions.1;

            let mut writing_i = (fold_y-1) as i32;
            for i in fold_y+1..paper.nrows() {
                for j in 0..paper.ncols() {
                    let writing_i = writing_i as usize;
                    paper[[writing_i, j]] = paper[[writing_i, j]].max(paper[[i, j]]);
                }
                writing_i -= 1;
            }

            return extract(&paper, 0, fold_y, 0, paper.ncols());

        }
        Axis::X => {
            let fold_x = instructions.1;

            let mut writing_j = (fold_x - 1) as i32;
            for j in fold_x+1..paper.ncols() {
                for i in 0..paper.nrows() {
                    let writing_j = writing_j as usize;
                    paper[[i, writing_j]] = paper[[i, writing_j]].max(paper[[i, j]]);
                }
                writing_j -= 1;
            }
            return extract(&paper, 0, paper.nrows(), 0, fold_x);
        }
    }
}

fn extract(paper: &Array2<u32>, start_y: usize, end_y: usize, start_x: usize, end_x: usize) -> Array2<u32> {
    let nrows = end_y - start_y;
    let ncols = end_x - start_x;

    let mut new_paper = Array2::zeros((nrows, ncols));

    for i in 0..nrows {
        for j in 0..ncols {
            new_paper[[i, j]] = paper[[start_y + i, start_x + j]];
        }
    }

    return new_paper;

}

fn read_file(path: &str) -> Array2<u32> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> Array2<u32> {
    let lines =  reader.lines().peekable();

    let mut max_x    = 0;
    let mut max_y    = 0;
    let mut dots_vec = Vec::<(u32, u32)>::new();
    for l in lines {
        let l = l.unwrap();

        let parsed = sscanf::scanf!(l, "{},{}", u32, u32);
        let points = parsed.unwrap();

        max_x = max_x.max(points.1);
        max_y = max_y.max(points.0);

        dots_vec.push(points);
    }

    let max_x = (max_x + 1) as usize;
    let max_y = (max_y + 1) as usize;

    let mut doc = Array2::default([max_x, max_y]);
    for dot in dots_vec {
        doc[[dot.1 as usize, dot.0 as usize]] = 1;
    }

    return doc
}



#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{Axis, fold, read_file, reader_to_data};

    #[test]
    fn example11() {
        let data =
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0";

        //fold along y=7
        //fold along x=5";
        //let instructions = vec![(Axis::Y, 7), (Axis::X, 5)];
        let instructions = vec![(Axis::Y, 7)];

        let mut reader = BufReader::new(data.as_bytes());
        let paper = reader_to_data(&mut reader);

        println!("{:?}", paper);

        let empty = fold(&paper, &instructions);
        assert_eq!(empty, 17);
    }

    #[test]
    fn example12() {
        let data =
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0";

        //fold along y=7
        //fold along x=5";
        let instructions = vec![(Axis::Y, 7), (Axis::X, 5)];

        let mut reader = BufReader::new(data.as_bytes());
        let paper = reader_to_data(&mut reader);

        println!("{:?}", paper);

        let empty = fold(&paper, &instructions);
        assert_eq!(empty, 16);
    }

    #[test]
    fn day13a() {
        let paper = read_file("src/day13.txt");

        // fold along x=655
        let instructions = vec![
            (Axis::X, 655),
        ];

        let empty = fold(&paper, &instructions);
        assert_eq!(empty, 795);
    }

    #[test]
    fn day13b() {
        let paper = read_file("src/day13.txt");

        // fold along x=655
        // fold along y=447
        // fold along x=327
        // fold along y=223
        // fold along x=163
        // fold along y=111
        // fold along x=81
        // fold along y=55
        // fold along x=40
        // fold along y=27
        // fold along y=13
        //  fold along y=6
        let instructions = vec![
            (Axis::X, 655),
            (Axis::Y, 447),
            (Axis::X, 327),
            (Axis::Y, 223),
            (Axis::X, 163),
            (Axis::Y, 111),
            (Axis::X, 81),
            (Axis::Y, 55),
            (Axis::X, 40),
            (Axis::Y, 27),
            (Axis::Y, 13),
            (Axis::Y, 6),
        ];

        let empty = fold(&paper, &instructions);
        assert_eq!(empty, 88);
    }
}
