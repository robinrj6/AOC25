use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

pub fn day01() -> io::Result<()> {
    // Start timer
    let start = Instant::now();
    // Read file input
    let file_str = "src/input1.txt";

    // Initialize constants
    let file = File::open(file_str)?;
    let reader = BufReader::new(file);

    // Mutable variables
    let mut position: i32 = 50;
    let mut zero_count: i32 = 0;

    // Loop over lines until there are no more
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Get the dial direction (L or R)
        let dir = line.chars().nth(0).unwrap();
        // Get the magnitude
        let val: i32 = line[1..].parse().expect("Unexpected integer read in input.");

        // Calculate full rotations and partial rotation
        let full = val / 100;
        let partial = val % 100;
        zero_count += full;

        // Calculate delta based on direction
        let delta = if dir == 'L' { -partial } else { partial };
        let next_position = position + delta;

        // Check if we cross zero during this rotation
        if position != 0 {
            if dir == 'L' && next_position <= 0 {
                zero_count += 1;
            } else if dir == 'R' && next_position >= 100 {
                zero_count += 1;
            }
        }

        // Update position with modulo
        position = next_position % 100;
        if position < 0 {
            position += 100;
        }
    }

    // Stop timer
    let elapsed = start.elapsed();
    // Print Results
    println!("Execution time (seconds): {:?}", elapsed.as_secs_f32());
    println!("final count: {}", zero_count);

    Ok(())
}