extern crate csv;

use std::io;
use csv::{Reader, ReaderBuilder};
use ndarray::{Array2, Zip};

fn main() {
    let commands = read_file("src/day2.txt");
    let final_pos = position_with_aim((0,0,0), &commands);
    println!("{} {}", final_pos.0, final_pos.1);
    println!("{}", final_pos.0 * final_pos.1);
}

// position(x,y) in a 2D plane.
fn position(initial_pos: (i32, i32), commands: &Array2<i32>) -> (i32, i32) {
    let mut final_pos = initial_pos.clone();

    Zip::from(commands.genrows())
        .apply(|r| {
            println!("{} {}", r[0], r[1]);

            final_pos.0 = final_pos.0 + r[0];
            final_pos.1 = final_pos.1 + r[1];

            // don't let get out of the water ;D
            if final_pos.1 < 0 {
                final_pos.1 = 0;
            }
        });


    return final_pos;
}

fn position_with_aim(initial_pos: (i32, i32, i32), commands: &Array2<i32>) -> (i32, i32, i32) {
    let mut final_pos = initial_pos.clone();

    Zip::from(commands.genrows())
        .apply(|r| {
            let mut x = final_pos.0;
            let mut y = final_pos.1;
            let mut aim = final_pos.2;

            // x y aim
            x = x + r[0];
            let y_drift = if r[0] > 0 { aim * r[0] } else { 0 };
            y = y + y_drift;

            aim = aim + r[1];

            // don't let get out of the water ;D
            if y < 0 {
                y = 0;
            }

            final_pos = (x, y, aim);

            println!("D: {} {} {}", x, y, aim);
        });


    return final_pos;
}

fn read_file(path: &str) -> Array2<i32> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(path).unwrap();

    return reader_to_commands(&mut reader);
}

fn reader_to_commands<R: io::Read>(reader: &mut Reader<R>) -> Array2<i32> {
    let mut commands = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        println!("{} {}", &record[0], &record[1]);

        let direction = &record[0];
        let magnitude = record[1].parse::<i32>().unwrap();



        let mut vector = (0, 0);
        match direction {
            "forward" => vector.0 = magnitude,
            "down" => vector.1 = magnitude,
            "up" => vector.1 = -magnitude,
            _ => panic!("Unknown direction {}", direction)
        }
        commands.push(vector);
    };

    let mut arr = Array2::<i32>::default((commands.len(), 2));
    for i in 0..commands.len() {
        arr[[i, 0]] = commands[i].0;
        arr[[i, 1]] = commands[i].1;
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;
    use crate::{position, position_with_aim, read_file, reader_to_commands};
    #[test]
    fn example1() {
        let data ="
forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let mut reader = ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(data.as_bytes());
        let commands = reader_to_commands(&mut reader);

        let final_pos = position((0,0), &commands);
        assert_eq!(final_pos.0 * final_pos.1, 150);
    }

    #[test]
    fn day2a() {
        let commands = read_file("src/day2.txt");
        let final_pos = position((0,0), &commands);
        assert_eq!(final_pos.0 * final_pos.1, 1762050);
    }

    #[test]
    fn example2() {
        let data ="
forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let mut reader = ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(data.as_bytes());
        let commands = reader_to_commands(&mut reader);

        let final_pos = position_with_aim((0,0,0), &commands);
        assert_eq!(final_pos.0 * final_pos.1, 900);
    }

    #[test]
    fn day2b() {
        let commands = read_file("src/day2.txt");

        let final_pos = position_with_aim((0,0,0), &commands);
        println!("{} {}", final_pos.0, final_pos.1);
        assert_eq!(final_pos.0 * final_pos.1, 1855892637);
    }
}

