extern crate csv;

use csv::ReaderBuilder;

fn main() {
    let data = read_file("src/day1.txt");
    let result = times_increased(&data);
}

fn times_increased(a: &Vec<i32>) -> u32 {
    if a.len() <= 0 {
        return 0;
    }

    let mut n_increased = 0;
    let mut previous : i32 = a[0];
    for i in 1..a.len() {
        let increased =  a[i] - previous > 0;
        n_increased = n_increased + (increased as u32);

        previous = a[i];
    }

    return n_increased;
}

fn sliding_window(a: &Vec<i32>, size: usize) -> Vec<i32> {
    let mut slided = Vec::new();

    if a.len() <= size {
        return slided;
    }

    let mut first = a[0];
    for i in 1..size {
        first = first + a[i];
    }
    slided.push(first);

    let mut window = first;
    for i in size..a.len() {
        window = window + a[i] - a[i-size];

        slided.push(window);
    }

    return slided;
}

fn read_file(path: &str) -> Vec<i32> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path).unwrap();

    let mut data = Vec::new();
    for record in reader.records() {
        let record = record.unwrap();
        let number_str = &record[0] ;
        let number = number_str.parse::<i32>().unwrap();
        println!("{}", number);
        data.push(number)

    }

    println!("Read {} numbers", data.len());

    return data;

}

#[cfg(test)]
mod tests {
    use crate::{read_file, sliding_window, times_increased};

    #[test]
    fn example1() {
        let data  = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
        let result = times_increased(&data);
        assert_eq!(result, 7);
    }

    #[test]
    fn day1a() {
        let data = read_file("src/day1.txt");
        let result = times_increased(&data);
        assert_eq!(result, 1706);
    }

    #[test]
    fn example2() {
        let data  = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
        let window = sliding_window(&data, 3);
        assert_eq!(window, [607, 618, 618, 617, 647, 716, 769, 792].to_vec());
        let result = times_increased(&window);
        assert_eq!(result, 5);

    }

    #[test]
    fn day1b() {
        let data = read_file("src/day1.txt");
        let result = times_increased(&data);
        assert_eq!(result, 1706);
    }
}