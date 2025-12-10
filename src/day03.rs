use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day03() -> io::Result<()> {
    println!("Running Day 03 solution...");
    let f = File::open("src/input3.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;

    // ### PART 1  ###
    // read a line into buffer
    // for line in reader.lines() {
    //     let line = line?;
    //     let line = line.trim();
    //     // let mut digit1=line.chars().nth(0).unwrap().to_digit(10).unwrap();
    //     // let mut digit2=line.chars().nth(1).unwrap().to_digit(10).unwrap();

    //     // process each line as a list of numbers
    //     // let numbersList: Vec<char> = line.chars().collect();
    //     let mut comp=0;
    //     for i in 0..(line.len()-1) {
    //         for j in (i + 1)..line.len() {
    //             let temp = (line.chars().nth(i).unwrap().to_digit(10).unwrap() *10)
    //                 + line.chars().nth(j).unwrap().to_digit(10).unwrap();
    //                 if temp > comp {
    //                     comp = temp;
    //                 }
    //         }
    //     }
    //     println!("Max two-digit number: {}", comp);
    //     result += comp;}

    // ### PART 2 ###
    // I need to find the 12 digit that combined creates the largest 12 digit number
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let mut calcnum:u128 = 0;
        let mut idx = 0;
        let mut max_digit = 0;
        let mut idx_new = 0;
        for i in 0..12 {
            (max_digit,idx_new) = find_max(&line[idx..], 12-i);
            calcnum = calcnum * 10 + max_digit as u128;
            idx += idx_new as usize + 1;

        }
        // println!(
        //         "Max digit found: {}, Current number: {}",
        //         max_digit, calcnum
        //     );
        result += calcnum;
    }

    println!("Day 03 result: {}", result);
    Ok(())
}

fn find_max(s: &str, idx_left: usize) -> (u32, u32) {
    let mut max_digit = 0;
    let mut index_max = 0;
    for i in 0..=(s.len() - idx_left) {
        let digit = s.chars().nth(i).unwrap().to_digit(10).unwrap();
        if digit > max_digit {
            max_digit = digit;
            index_max = i;
        }
    }
    (max_digit, index_max as u32)
}
