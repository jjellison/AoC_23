use std::fs::File;
use std::io::{self, BufRead, BufReader};


pub fn read_lines(filepath: &str) -> io::Result<io::Lines<BufReader<File>>> {

    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}


pub fn num_lines(filepath: &str) -> io::Result<usize> {

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    
    Ok(reader.lines().count())
}
