pub fn day02() -> std::io::Result<()> {
    let mut result = 0;
    let input = "194-253,81430782-81451118,7709443-7841298,28377-38007,6841236050-6841305978,2222204551-2222236166,2623-4197,318169-385942,9827-16119,580816-616131,646982-683917,147-181,90-120,3545483464-3545590623,4304-5747,246071-314284,8484833630-8484865127,743942-795868,42-53,1435-2086,50480-60875,16232012-16441905,94275676-94433683,61509567-61686956,3872051-4002614,6918792899-6918944930,77312-106847,282-387,829099-1016957,288251787-288311732,6271381-6313272,9877430-10095488,59-87,161112-224439,851833788-851871307,6638265-6688423,434-624,1-20,26-40,6700-9791,990-1307,73673424-73819233";

    let mut valid_ranges = Vec::new();
    for pair in input.split(',') {
        let bounds: Vec<&str> = pair.split('-').collect();
        let lower: String = bounds[0].parse().unwrap();
        let upper: String = bounds[1].parse().unwrap();
        // ### PART 1 ###
        // if lower.len() % 2 != 0 && upper.len() % 2 != 0 {
        //     continue;
        // }
        valid_ranges.push((lower, upper));
    }
    // println!("{:?}", valid_ranges);
    for (lower, upper) in valid_ranges {
        let lowerInt: u64 = lower.parse().unwrap();
        let upperInt: u64 = upper.parse().unwrap();
        // ### PART 1 ###
        // for num in lowerInt..=upperInt {
        //     let numStr = num.to_string();
        //     if numStr[0..(numStr.len() / 2)] == numStr[(numStr.len() / 2)..] {
        //         result += num
        //     } else {
        //         continue;
        //     }
        // }
        // ### PART 2 ###
        for num in lowerInt..=upperInt {
            let numStr = num.to_string();

            //check for substrings of increasing length till half the string length
            for subLen in 1..=(numStr.len() / 2) {
                let sub_str = &numStr[0..subLen];
                // Check if repeating sub_str forms the entire string
                let mut is_repeating = true;
                for i in (0..numStr.len()).step_by(subLen) {
                    let end = (i + subLen).min(numStr.len());
                    if &numStr[i..end] != &sub_str[0..(end - i)] {
                        is_repeating = false;
                        break;
                    }
                }
                if is_repeating && numStr.len() % subLen == 0 {
                    result += num;
                    break;
                }
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}