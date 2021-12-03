extern crate csv;

use std::io;
use csv::{Reader, ReaderBuilder};
use ndarray::{Array1, Array2, Axis, Zip};

fn main() {
    let data = read_file("src/day3.txt");

    let (epsilon, gamma) = epsilon_gamma(&data);
    println!("epsilon gamma: {}", epsilon*gamma);

    let co2 = generator(&data, RatioType::CO2);
    let o2 = generator(&data, RatioType::OXYGEN);

    println!("co2 o2: {}", co2*o2);
}

fn epsilon_gamma(data: &Array2<u8>) -> (u32, u32) {
    println!("{}", data);

    let n_samples = data.shape()[0];
    let sample_size = data.shape()[1];
    let mut count = Array1::<u32>::default(sample_size);

    Zip::from(data.genrows())
        .apply(|r| {

            for i in 0..count.len() {
                count[i] = count[i] + r[i] as u32
            }
        });

    println!("{}", count);

    let threshold = n_samples as f32 / 2.0;
    let max = count.map(|x| if *x as f32 >= threshold { 1 } else { 0 });
    let min = max.map(|x| if *x == 1 { 0 } else { 1 });

    println!("{}", max);
    println!("{}", min);

    let gamma = boolean_to_decimal(&max);
    let epsilon = boolean_to_decimal(&min);

    return (gamma, epsilon);
}

enum RatioType {
    OXYGEN,
    CO2,
}

fn generator(data: &Array2<u8>, ratio: RatioType) -> u32 {
    let n_samples = data.shape()[0];
    let sample_size = data.shape()[1];


    let mut remaining_rows : Vec<usize> = (0..n_samples).collect();
    for j in 0..sample_size {

        let mut count = 0;
        for r in 0..remaining_rows.len() {
            let i = remaining_rows[r];
            count = count + data[[i,j]] as u32;
        }

        let threshold = remaining_rows.len() as f32 / 2.0;
        let count = count as f32;
        let mode =
            match ratio {
                RatioType::OXYGEN => {
                    if count >= threshold { 1 } else { 0 }

                },
                RatioType::CO2 => {
                    if count < threshold { 1 } else { 0 }
                },
            };

        remaining_rows.retain(|r| { data[[*r, j]] == mode });

        if remaining_rows.len() == 1 {
            let rem = remaining_rows[0] as usize;
            let final_row = data.row(rem).to_owned();
            return boolean_to_decimal(&final_row);
        }
    }

    return 0;
}

fn boolean_to_decimal(booleans: &Array1<u8>) -> u32 {
    let mut pow = 1;
    let mut decimal = 0;
    for i in (0..booleans.len()).rev() {
        if booleans[i] == 1 {
            decimal = decimal + pow;
        }
        pow = pow * 2;
    }

    return decimal;
}

fn read_file(path: &str) -> Array2<u8> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(path).unwrap();

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut Reader<R>) -> Array2::<u8> {
    let mut data = Vec::new();

    let mut n = 0;
    for record in reader.records() {
        let record = record.unwrap();

        let row = &record[0];
        let row: Vec<u8> = row.chars().map(|x| if '0' == x { 0 } else { 1 } ).collect();

        if n == 0 {
            n = row.len();
        }

        data.push(row);

    };

    let mut arr = Array2::<u8>::default((data.len(), n));
    for i in 0..data.len() {
        for j in 0..n {
            arr[(i, j)] = data[i][j];
        }
    }

    return arr;
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;
    use crate::{epsilon_gamma, generator, RatioType, read_file, reader_to_data};
    #[test]
    fn example1() {
        let data =
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let mut reader = ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let (epsilon, gamma) = epsilon_gamma(&data);
        assert_eq!(epsilon * gamma, 198);
    }

    #[test]
    fn day3a() {
        let data = read_file("src/day3.txt");

        let (epsilon, gamma) = epsilon_gamma(&data);
        assert_eq!(epsilon * gamma, 3549854);
    }

    #[test]
    fn example2() {
        let data =
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let mut reader = ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let o2 = generator(&data, RatioType::OXYGEN);
        let co2 = generator(&data, RatioType::CO2);
        assert_eq!(co2 * o2, 230);
    }

    #[test]
    fn day3b() {
        let data = read_file("src/day3.txt");

        let co2 = generator(&data, RatioType::CO2);
        let o2 = generator(&data, RatioType::OXYGEN);

        assert_eq!(co2 * o2, 3765399);
    }
}

