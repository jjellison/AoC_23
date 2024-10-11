use crate::utils;
use std::io;


const INPUT_FILE_PATH: &str = "src/day1/input.txt";
const TENS_COLUMN: usize = 0;
const ONES_COLUMN: usize = 1;

pub fn run() -> io::Result<u32,> {
    let mut file_lines = utils::read_lines(INPUT_FILE_PATH)?;

    let sum = file_lines.by_ref()
        .filter_map(Result::ok)
        .map(|valid_line| 
                valid_line.chars()
                .filter_map(|c| 
                    c.to_digit(10))
                .fold([None; 2], |mut state, digit| {
                    state[TENS_COLUMN].get_or_insert(digit);
                    state[ONES_COLUMN] = Some(digit);
                    state
                }))
        .filter_map(|pair| {
                pair.iter()
                    .try_fold(0, |working, digit_or_none|
                        digit_or_none.map(|digit| (working * 10) + digit))
        })
        .sum();
            
    Ok(sum)
}
