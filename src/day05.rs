use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::result;

pub fn day05() -> io::Result<()> {
    println!("Running Day 05 solution...");
    let f = File::open("src/input5.txt")?;
    let mut reader = BufReader::new(f);
    let mut result: i32 = 0;

    let (mut start, mut finish) = (0, 0);
    let mut validNums: Vec<(usize, usize)> = Vec::new();
    let mut validNums2: Vec<(usize, usize)> = Vec::new();

    let mut flag = true;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            flag = false;
            break;
        }
        if flag == true {
            (start, finish) = (
                line.split('-').nth(0).unwrap().parse::<usize>().unwrap(),
                line.split('-').nth(1).unwrap().parse::<usize>().unwrap(),
            );

            validNums.push((start, finish));

            // println!("Valid numbers from {} to {} stored.", start, finish);
        }
        // ### PART 1 ###
        // else{
        //     let num = line.parse::<usize>().unwrap();
        //     let mut isValid = false;
        //     for (s, f) in &validNums {
        //         if num >= *s && num <= *f {
        //             isValid = true;
        //             break;
        //         }
        //     }
        //     if isValid {
        //         result += 1;
        //     }
        // }
    }


    println!("{:?}", validNums);
    // println!("Result: {}", result);
    Ok(())
}
