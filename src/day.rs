use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day__() -> io::Result<()> {
    let f = File::open("src/input_.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;
    for line in reader.lines() {
        let line = line?;



    }

    Ok(())
}