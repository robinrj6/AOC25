use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day06() -> io::Result<()> {
    let f = File::open("src/input6.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;

    let mut size: usize = 0;
    let mut lists: Vec<String> = Vec::new();
    let mut num_lists: Vec<Vec<u64>> = Vec::new();

    let mut collect: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lists = line
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        collect.push(lists.clone());
    }

    println!("Total lines read: {:?}", collect[collect.len() - 1]);
    for i in 0..collect.len() - 1 {
        num_lists.push(
            collect[i]
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
        );
    }
    println!("Number lists: {:?}", num_lists);
    let mut calc = 0;
    for i in 0..num_lists[0].len() {
        let mut temp = 0;
        for j in 0..num_lists.len() {
            if collect[collect.len() - 1][i] == "*" {
                if j==0 {
                    calc = 1;
                }   
                calc = calc * num_lists[j][i];
            } else if collect[collect.len() - 1][i] == "+" {
                calc = calc + num_lists[j][i];
            } else if collect[collect.len() - 1][i] == "-" {
                calc = calc - num_lists[j][i];
            } else if collect[collect.len() - 1][i] == "/" {
                calc = calc / num_lists[j][i];
            }
        }
        println!("Calculation result: {}", calc);
        result += calc;
        calc = 0;
    }
    
    println!("Day 06 result: {}", result);
    Ok(())
}
