// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

// // Single-digit hex insertion; no external crate needed

// pub fn day07() -> io::Result<()> {
//     let f = File::open("src/input7.txt")?;
//     let mut reader = BufReader::new(f);
//     let mut result:i64 = 0;
//     let mut lines: Vec<String> = Vec::new();
//     for line in reader.lines() {
//         let line = line?;
//         lines.push(
//             line.clone()
//                 .split_whitespace()
//                 .map(|s| s.to_string())
//                 .collect(),
//         );
//     }

//     // println!("Lists: {:?}", lines[0]);
//     //find position of "S" in the first list
//     let start_pos = findSposition(lines[0].clone());
//     println!("Start position of 'S': {}", start_pos);
//     // Write the value 1 as a single hex digit ("1")
//     let val: u8 = 1;
//     let hex_char = std::char::from_digit(val as u32, 16).unwrap();
//     let mut next_line_chars: Vec<char> = lines.get(1).cloned().unwrap_or_default().chars().collect();
//     if start_pos < next_line_chars.len() {
//         next_line_chars[start_pos] = hex_char;
//     }
//     lines[1] = next_line_chars.into_iter().collect();

    

//     // println!("Modified Lines: {:?}", lines);
//     // ## PART 2 ##
//     for i in 0..lines.len() - 1 {
//         for j in 0..lines[i].len() {
//             if lines[i]
//                 .chars()
//                 .nth(j)
//                 .map(|c| c.is_ascii_hexdigit())
//                 .unwrap_or(false)
//             {
//                 if lines[i + 1].chars().nth(j) == Some('^') {
//                     let mut chars: Vec<char> = lines[i+1].chars().collect();
//                     if chars[j-1]=='.' {
//                         chars[j-1] = '0';
//                         chars[j-1] = inc_hex_char(chars[j-1],lines[i].chars().nth(j).unwrap());
//                     }
//                     else {chars[j-1] = inc_hex_char(chars[j-1],lines[i].chars().nth(j).unwrap());}
//                     if chars[j+1]=='.' {
//                         chars[j+1] = '0';
//                         chars[j+1] = inc_hex_char(chars[j+1],lines[i].chars().nth(j).unwrap());
//                     }
//                     else {chars[j+1] = inc_hex_char(chars[j+1],lines[i].chars().nth(j).unwrap());}

//                     lines[i+1] = chars.iter().collect();
//                 } 
//                 else {
//                     let mut chars: Vec<char> = lines[i + 1].chars().collect();
//                     if chars[j].is_ascii_hexdigit() {
//                         chars[j] = inc_hex_char(chars[j], lines[i].chars().nth(j).unwrap());
//                     } else {
//                         chars[j] = lines[i].chars().nth(j).unwrap();
//                     }
//                     lines[i + 1] = chars.iter().collect();
//                 }
//             }
//         }
//         println!("Line  {:?}", lines[i]);
//     }

//     // ## END PART 2 ##
//     //## PART 1 ##
//     // for i in 0..lines.len()-1 {
//     //     for j in 0..lines[i].len()  {
//     //         if lines[i].chars().nth(j) == Some('|') {
//     //             if lines[i+1].chars().nth(j) == Some('^') {
//     //                 result += 2;
//     //                 let mut chars: Vec<char> = lines[i+1].chars().collect();
//     //                     chars[j-1] = '|';
//     //                     chars[j+1] = '|';
//     //                 lines[i+1] = chars.iter().collect();
//     //             }
//     //             else {
//     //                 let mut chars: Vec<char> = lines[i+1].chars().collect();
//     //                 chars[j] = '|';
//     //                 lines[i+1] = chars.iter().collect();
//     //             }
//     //         }
//     //     }
//     //     println!("Line {}: {:?}", i, lines[i]);
//     // }
//     //## END PART 1 ##
//     for i in 0..lines[lines.len()-1].len() {
//         if lines[lines.len()-1].chars().nth(i).map(|c| c.is_ascii_hexdigit()).unwrap_or(false) {
//             result += lines[lines.len()-1].chars().nth(i).unwrap().to_digit(16).unwrap() as i64;
//             println!("Adding {}, Result: {}", lines[lines.len()-1].chars().nth(i).unwrap(), result);
//         }
//     }
//     println!("Result: {}", result);
//     Ok(())
// }

// fn findSposition(rows: String) -> usize {
//     if let Some(col_idx) = rows.find('S') {
//         return col_idx;
//     }
//     0 // Return max usize if not found
// }

// fn inc_hex_char(c: char, increment_char: char) -> char {
//     match c {
//         '0'..='9' | 'a'..='f' | 'A'..='F' => {
//             let upper = c.is_ascii_uppercase();
//             let val = c.to_digit(16).unwrap();
//             let new = (val + increment_char.to_digit(16).unwrap()) % 16;
//             let mut ch = std::char::from_digit(new, 16).unwrap();
//             if upper {
//                 ch = ch.to_ascii_uppercase();
//             }
//             ch
//         }
//         _ => c,
//     }
// }
use std::collections::HashMap;

pub fn day07() {
    // Read input from stdin (you can also read from file)
    let input = std::fs::read_to_string("src/input7.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    // Find starting column
    let start_row = lines[0];
    let start_col = start_row.find('S').unwrap();

    // paths: map of (column -> number of paths reaching this column)
    let mut paths: HashMap<usize, u128> = HashMap::new();
    paths.insert(start_col, 1);

    // For every subsequent row
    for row in &lines[1..] {
        let mut next_paths: HashMap<usize, u128> = HashMap::new();

        for (&col, &count) in paths.iter() {
            let row_chars: Vec<char> = row.chars().collect();

            // If this column is within bounds
            if col < row_chars.len() {
                if row_chars[col] == '^' {
                    // Split into left/right
                    if col > 0 {
                        *next_paths.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < row_chars.len() {
                        *next_paths.entry(col + 1).or_insert(0) += count;
                    }
                } else {
                    // Straight down (no splitter)
                    *next_paths.entry(col).or_insert(0) += count;
                }
            }
        }

        // Move to next row
        paths = next_paths;
    }

    // Sum up all path counts at the bottom row
    let result: u128 = paths.values().sum();
    println!("Part 2 result: {}", result);
}
