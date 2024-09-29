use crate::utils;
use std::io;


const INPUT_FILE_PATH: &str = "src/day1/input.txt";
const TENS_COLUMN: usize = 0;
const ONES_COLUMN: usize = 1;

pub fn run() -> io::Result<u32,> {
    let file_lines = utils::read_lines(INPUT_FILE_PATH)?;

    let sum = file_lines
        .filter_map(|line| line.ok())
        .map(|valid_line| 
                valid_line.chars()
                .filter_map(|c| 
                    c.to_digit(10))
                .fold([None; 2], |mut state, digit| {
                    if state[TENS_COLUMN].is_none() {
                        state[TENS_COLUMN] = Some(digit);
                    }
                    state[ONES_COLUMN] = Some(digit);
                    state
                }))
        .filter_map(|pair| {
            if pair.iter().all(|&digit| digit.is_some()) {
                Some(pair[TENS_COLUMN].unwrap() * 10 + (pair[ONES_COLUMN].unwrap()))
            }
            else {
                None
            }})
        .sum();
            
    Ok(sum)
}
