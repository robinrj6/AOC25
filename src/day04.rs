use core::str;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day04() -> io::Result<()> {
    println!("Running Day 04 solution...");
    let f = File::open("src/input4.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;
    let mut temp = 0; // ## part2 ##

    // Your Day 04 solution logic goes here
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    // ## part2 ##
    while true {
    // ## part2 ##
        temp = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                // Example processing logic
                let current_char = grid[r][c];
                if current_char == '@' {
                    // Do something with '@'
                    if checker8adj(&grid, r, c) == true {
                        result += 1;
                        // ## part2 ##
                        temp += 1;
                        grid[r][c] = 'x';
                        // ## part2 ##
                    }
                }
            }
        }
        //## part2 ##
        if temp == 0 {
            break;
        }// ## part2 ##
    }
    println!("Day 04 result: {}", result);
    Ok(())
}

fn checker8adj(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_r = r as isize + dr;
        let new_c = c as isize + dc;
        if new_r >= 0 && new_r < grid.len() as isize && new_c >= 0 && new_c < grid[0].len() as isize
        {
            if grid[new_r as usize][new_c as usize] == '@' {
                count += 1;
            }
        }
    }
    if count < 4 {
        return true;
    }
    false
}
