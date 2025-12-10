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
    let mut lowest12digit ="100000000000";
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let mut calcnum=0;
        let mut idx=0;
        for i in idx..(line.len()-12) { 
            // println!("Current 12-digit substring: {}", &line[0..line.len()-12]);
            let substring = &line[idx..line.len()-12];
            // Find the biggest single digit in this substring
            let mut max_digit = 0;
            // substring.chars()
            //     .filter_map(|c| c.to_digit(10))
            //     .max()
            //     .unwrap_or(0);
            let mut max_index = 0;
            for (idxt, c) in substring.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if digit > max_digit {
                        max_digit = digit;
                        max_index = idxt;
                    }
                }
            }
            if idx==max_index{
                idx +=1;
            } else {
                idx = max_index;
            }
            calcnum = calcnum * 10 + max_digit;
            println!("Max digit found: {}, Current number: {}", max_digit, calcnum);
            // break;
        }
        

        
    }

    println!("Day 03 result: {}", result);
    Ok(())
}
