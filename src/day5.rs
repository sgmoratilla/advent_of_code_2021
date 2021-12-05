use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use ndarray::{Array2};

fn main() {
    let (max, vents_lines) = read_file("src/day5.txt");

    let vents_map = vents_map(&max, &vents_lines, false);
    let overlaps = overlaps(&vents_map);

    println!("overlaps: {}", overlaps);
}

fn vents_map(max_movements: &(u32, u32), vents_lines: &Vec<(u32, u32, u32, u32)>, only_straight_lines: bool) -> Array2::<u32> {
    let width = (max_movements.0 + 1) as usize;
    let depth = (max_movements.1 + 1) as usize;
    let mut vents_map = Array2::<u32>::zeros([width, depth]);

    for l in vents_lines {
        if !only_straight_lines || straight_line(l) {
            draw(l, &mut vents_map);
        }
    }

    println!("{}", vents_map);
    return vents_map;
}

fn draw(l: &(u32, u32, u32, u32), vents_map: &mut Array2::<u32>) {
    let y_range = (l.0 as i32 - l.2 as i32).abs() + 1;
    let x_range = (l.1 as i32 - l.3 as i32).abs() + 1;
    let size = y_range.max(x_range);

    let x_dir: i32 = compute_direction(l.1, l.3);
    let y_dir: i32 = compute_direction(l.0, l.2);

    println!("{} {} -> {} {} | {} {}", l.0, l.1, l.2, l.3, y_dir, x_dir);

    let mut y = l.0 as i32;
    let mut x  = l.1 as i32;
    for i in 0..size {
        vents_map[[x as usize, y as usize]] += 1;
        x += x_dir;
        y += y_dir;
    }
}

fn compute_direction(begin: u32, end: u32) -> i32 {
    if begin == end {
        return 0;
    } else if begin > end {
        return -1;
    } else {
        return 1;
    }
}

fn straight_line(line: &(u32, u32, u32, u32)) -> bool {
    return line.0 == line.2 || line.1 == line.3;
}

fn overlaps(vents_map: &Array2::<u32>) -> u32 {
    vents_map.fold(0, |acc, v| {
        if *v > 1 {
            return acc + 1;
        }
        return acc;
    })
}



fn read_file(path: &str) -> ((u32, u32), Vec<(u32, u32, u32, u32)>) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> ((u32, u32), Vec<(u32, u32, u32, u32)>) {
    let lines =  reader.lines().peekable();

    let mut max_x    = 0;
    let mut max_y    = 0;
    let mut vents_lines = Vec::<(u32, u32, u32, u32)>::new();
    for l in lines {
        let l = l.unwrap();
        let parsed = sscanf::scanf!(l, "{},{} -> {},{}", u32, u32, u32, u32);
        // x0 y0 -> x1 y1
        let direction = parsed.unwrap();

        //println!("{},{} -> {},{}", direction.0, direction.1, direction.2, direction.3);

        max_x = max_x.max(direction.0.max(direction.2));
        max_y = max_y.max(direction.1.max(direction.3));
        vents_lines.push(direction);
    }

    return ((max_x, max_y), vents_lines);
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{overlaps, read_file, reader_to_data, vents_map};

    #[test]
    fn example1() {
        let data =
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let mut reader = BufReader::new(data.as_bytes());
        let (max, vents_lines) = reader_to_data(&mut reader);

        let vents_map = vents_map(&max, &vents_lines, true);
        let overlaps = overlaps(&vents_map);

        assert_eq!(overlaps, 5);
    }

    #[test]
    fn day5a() {
        let (max, vents_lines) = read_file("src/day5.txt");

        let vents_map = vents_map(&max, &vents_lines, true);
        let overlaps = overlaps(&vents_map);

        assert_eq!(overlaps, 7269);
    }

    #[test]
    fn example2() {
        let data =
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let mut reader = BufReader::new(data.as_bytes());
        let (max, vents_lines) = reader_to_data(&mut reader);

        let vents_map = vents_map(&max, &vents_lines, false);
        let overlaps = overlaps(&vents_map);

        assert_eq!(overlaps, 12);
    }

    #[test]
    fn day5b() {
        let (max, vents_lines) = read_file("src/day5.txt");

        let vents_map = vents_map(&max, &vents_lines, false);
        let overlaps = overlaps(&vents_map);

        assert_eq!(overlaps, 7269);
    }

}

