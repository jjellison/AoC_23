use crate::utils;

mod part1;
mod part2;


pub fn run() {
    println!("Day 1:");

    utils::format_run(1, part1::run);
    utils::format_run(2, part2::run);
}
