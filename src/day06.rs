use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day06() -> io::Result<()> {
    let f = File::open("src/input6.txt")?;
    let mut reader = BufReader::new(f);
    let mut result = 0;

    let mut lists: Vec<String> = Vec::new();
    let mut num_lists: Vec<Vec<u64>> = Vec::new();
    // Preserve raw lines (including indentation) for vertical reading
    let mut raw_lines: Vec<String> = Vec::new();

    let mut collect: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // Keep indentation for vertical parsing
        raw_lines.push(line.clone());
        // Token view still useful for row-wise operations
        lists = line.split_whitespace().map(|s| s.to_string()).collect();
        collect.push(lists.clone());
    }

    // ### PART 2 ###
    let mut signs = raw_lines.pop().unwrap_or_default();
    // print!("Signs: {:?}", signs);
    //transpose the raw_lines into columns
    let mut columns: Vec<String> = Vec::new();
    let max_length = raw_lines.iter().map(|s| s.len()).max().unwrap_or(0);
    for i in 0..max_length {
        let mut col = String::new();
        for line in &raw_lines {
            if let Some(ch) = line.chars().nth(i) {
                col.push(ch);
            }
        }
        columns.push(col);
    }
    // Build a typed view where pure whitespace entries are distinguishable
    // from numeric entries. Numeric strings are parsed; whitespace-only -> None.
    let typed_columns: Vec<Option<u64>> = columns
        .iter()
        .map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else if trimmed.chars().any(|c| c.is_ascii_digit()) {
                trimmed.parse::<u64>().ok()
            } else {
                None
            }
        })
        .collect();

    let mut countSIgns = 0;
    let mut calc = 0;
    // Demo output to show distinction
    signs = signs.split_whitespace().collect::<Vec<&str>>().join("");
    println!("Signs: {:?}", signs);
    for i in 0..typed_columns.len() {
        if let Some(num) = typed_columns[i] {
            let sign = signs.chars().nth(countSIgns).unwrap_or(' ');
            if sign == '+' {
                calc += num;
            } else if sign == '*' {
                if calc == 0 {
                    calc = 1;
                }
                calc *= num;
            } else if sign == '-' {
                calc -= num;
            } else if sign == '/' {
                calc /= num;
            }
            println!(
                "Column {}: Number = {}, Sign = '{}', Calc = {}",
                i, num, sign, calc
            );
        } else {
            result += calc;
            calc = 0;
            countSIgns += 1;
            // It's whitespace or non-numeric
            continue;
        }
    }
    result += calc;

    //### END PART 2 ###

    // ### PART 1 ###
    // println!("Total lines read: {:?}", collect[collect.len() - 1]);
    // for i in 0..collect.len() - 1 {
    //     num_lists.push(
    //         collect[i]
    //             .iter()
    //             .map(|s| s.parse::<u64>().unwrap())
    //             .collect(),
    //     );
    // }
    // // print!("Number lists: {:?}", num_lists);
    // let mut calc = 0;
    // for i in 0..num_lists[0].len() {
    //     let mut temp = 0;
    //     for j in 0..num_lists.len() {
    //         if collect[collect.len() - 1][i] == "*" {
    //             if j==0 {
    //                 calc = 1;
    //             }
    //             calc = calc * num_lists[j][i];
    //         } else if collect[collect.len() - 1][i] == "+" {
    //             calc = calc + num_lists[j][i];
    //         } else if collect[collect.len() - 1][i] == "-" {
    //             calc = calc - num_lists[j][i];
    //         } else if collect[collect.len() - 1][i] == "/" {
    //             calc = calc / num_lists[j][i];
    //         }
    //     }
    //     // println!("Calculation result: {}", calc);
    //     result += calc;
    //     calc = 0;
    // }
    // ### END PART 1 ###

    println!("Day 06 result: {}", result);
    Ok(())
}
