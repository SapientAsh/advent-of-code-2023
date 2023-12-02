use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("SecondaryFiles/input.txt").unwrap();
    let reader = BufReader::new(file);

    let values: Vec<i32> = reader.lines().map( | line | {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.chars().filter( | char | {
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(char)
        })
        .map( | char | char as i32 - 48)
        .collect();
        numbers[0] * 10 + numbers[numbers.len()-1]
    })
    .collect();
    let part1: i32 = values.iter().sum();
    let part2 = 0;
    println!("Part 1: {} \nPart 2: {}", part1, part2);
}
