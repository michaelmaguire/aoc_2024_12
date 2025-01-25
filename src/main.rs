use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_matrix_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(matrix)
}

fn main() {
    println!("Hello, aoc_2024_12!");

    let filename = "./src/input.txt";
    match read_matrix_from_file(filename) {
        Ok(matrix) => {
            for row in matrix {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}