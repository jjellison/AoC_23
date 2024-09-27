use crate::utils;


const INPUT_FILE_PATH: &str = "./input.txt";

pub fn run() -> io::Result<()> {
    println!("  Part 1");

    let file_lines = utils::read_lines(INPUT_FILE_PATH)?;
    
    Ok(())
}
