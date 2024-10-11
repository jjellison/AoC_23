use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

static NUMBER_STRINGS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn read_lines(filepath: &str) -> io::Result<io::Lines<BufReader<File>>> {

    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}



pub fn convert_strings_to_digits(line: &str) -> impl Iterator<Item = u32> +'_{
    
    line.char_indices().filter_map(|(start, c)| {
        c.to_digit(10).or_else(||{
            NUMBER_STRINGS
                .iter()
                .enumerate()
                .filter_map(|(i, &number)| {
                    let end = start + number.len();
                    let window = line.get(start..end);
                    window.and_then(|window_str| (window_str == number).then_some(i as u32))
                })
                .next()
        })
    })
}

pub fn format_run<T>(part: usize, f: fn() -> io::Result<T>) where T:std::fmt::Display {
    
    let start = Instant::now();
    let result = f();
    let delta = (Instant::now() - start).as_micros();

    match result {
        Ok(value) => { println!("  Part {}: {}, Time: {}us", part, value, delta); }
        Err(e) => { println!(" Part {}: Error {}", part, e); }
    };


}
