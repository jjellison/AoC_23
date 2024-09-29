

mod part1;
mod part2;


pub fn run() {
    println!("Day 1:");
    print!("  Part1: ");
    match part1::run() {
        Ok(value) => { println!("{}", value); }
        Err(e) => { println!("Day1 Part1 faild. Error {}", e); }
    };

    print!("  Part 2:");
    match part2::run() {
        Ok(value) => { println!("{}", value); }
        Err(e) => { println!("FAILED! Error {}", e)}
    };
}
