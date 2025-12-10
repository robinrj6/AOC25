use core::str;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

pub fn day04() -> io::Result<()> {
    println!("Running Day 04 solution...");
    let f = File::open("src/input4.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;
    let mut temp = 0;

    // Your Day 04 solution logic goes here
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    while true {
        temp = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                // Example processing logic
                let current_char = grid[r][c];
                if current_char == '@' {
                    // Do something with '@'
                    if checker8adj(&grid, r, c) == true {
                        result += 1;
                        temp += 1;
                        grid[r][c] = 'x';
                    }
                }
            }
        }
        if temp == 0 {
            break;
        }
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

// fn checker8adj(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
//     let mut count = 0;
//     let directions = [
//         (-1, -1),
//         (-1, 0),
//         (-1, 1),
//         (0, -1),
//         (0, 1),
//         (1, -1),
//         (1, 0),
//         (1, 1),
//     ];
//     let noTopDirection = [
//         (0, -1),
//         (0, 1),
//         (1, -1),
//         (1, 0),
//         (1, 1),
//     ];
//     let noBtmDirection = [
//         (-1, -1),
//         (-1, 0),
//         (-1, 1),
//         (0, -1),
//         (0, 1),
//     ];
//     let noLeftDirection = [
//         (-1, 0),
//         (-1, 1),
//         (0, 1),
//         (1, 0),
//         (1, 1),
//     ];
//     let noRightDirection = [
//         (-1, -1),
//         (-1, 0),
//         (0, -1),
//         (1, -1),
//         (1, 0),
//     ];
//     let lefttopedge = [
//         (0, 1),
//         (1, 0),
//         (1, 1),
//     ];
//     let righttopedge = [
//         (0, -1),
//         (1, -1),
//         (1, 0),
//     ];
//     let leftbtmedge = [
//         (-1, 0),
//         (-1, 1),
//         (0, 1),
//     ];
//     let rightbtmedge = [
//         (-1, -1),
//         (-1, 0),
//         (0, -1),
//     ];

//     for (dr, dc) in directions.iter() {
//         let new_r = r as isize + dr;
//         let new_c = c as isize + dc;
//         if new_r >= 0
//             && new_r < grid.len() as isize
//             && new_c >= 0
//             && new_c < grid[0].len() as isize
//         {
//             if grid[new_r as usize][new_c as usize] == '@' {
//                 count += 1;
//             }
//         }
//         if
//         if r==0  {
//             for (dr, dc) in noTopDirection.iter() {
//                 let new_r = r as isize + dr;
//                 let new_c = c as isize + dc;
//                 if new_r >= 0
//                     && new_r < grid.len() as isize
//                     && new_c >= 0
//                     && new_c < grid[0].len() as isize
//                 {
//                     if grid[new_r as usize][new_c as usize] == '@' {
//                         count += 1;
//                     }
//                 }
//             }
//             break;
//         }
//         if r==grid.len()-1  {
//             for (dr, dc) in noBtmDirection.iter() {
//                 let new_r = r as isize + dr;
//                 let new_c = c as isize + dc;
//                 if new_r >= 0
//                     && new_r < grid.len() as isize
//                     && new_c >= 0
//                     && new_c < grid[0].len() as isize
//                 {
//                     if grid[new_r as usize][new_c as usize] == '@' {
//                         count += 1;
//                     }
//                 }
//             }
//             break;
//         }
//         if c==0  {
//             for (dr, dc) in noLeftDirection.iter() {
//                 let new_r = r as isize + dr;
//                 let new_c = c as isize + dc;
//                 if new_r >= 0
//                     && new_r < grid.len() as isize
//                     && new_c >= 0
//                     && new_c < grid[0].len() as isize
//                 {
//                     if grid[new_r as usize][new_c as usize] == '@' {
//                         count += 1;
//                     }
//                 }
//             }
//             break;
//         }
//         if c==grid[0].len()-1  {
//             for (dr, dc) in noRightDirection.iter() {
//                 let new_r = r as isize + dr;
//                 let new_c = c as isize + dc;
//                 if new_r >= 0
//                     && new_r < grid.len() as isize
//                     && new_c >= 0
//                     && new_c < grid[0].len() as isize
//                 {
//                     if grid[new_r as usize][new_c as usize] == '@' {
//                         count += 1;
//                     }
//                 }
//             }
//             break;
//         }
//     }
//     if count>4 {
//         return false;
//     }
//     true
// }
