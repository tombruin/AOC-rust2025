advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut max_output_joltage: u64 = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line.chars()
            .filter_map(|ch| ch.to_digit(10)).collect();

        let mut remaining = digits.clone();
        let mut result: Option<(u32, u32)> = None;

        while !remaining.is_empty() {
            // Find max1 and its position. Make sure to take the first highest number. (sort by index)
            if let Some((pos1, &max1)) = remaining.iter().enumerate().max_by_key(|&(i, d)| (d, -(i as isize))) {
                // Try to find max2 after pos1
                if let Some(&max2) = digits.iter().skip(pos1 + 1).max() {
                    result = Some((max1, max2));
                    break;
                } else {
                    // Remove this candidate and try again to find new max1
                    remaining.retain(|&d| d != max1);
                }
            }
        }

        match result {
            Some((max1, max2)) => {
                max_output_joltage += ((max1 * 10) + max2) as u64;
            }
            None => println!("No valid min/max pair found"),
        }

    }
    return Some(max_output_joltage)
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut max_output_joltage: u64 = 0;
    let digit_max_count = 12; // how many digits we want

    for line in input.lines() {
        let digits: Vec<u32> = line.chars()
            .filter_map(|ch| ch.to_digit(10)).collect();
        let mut combined_bank_joltage: u64 = 0;
        let mut digit_count = 0;
        let mut start = 0;

        while digit_count < digit_max_count && start < digits.len() {
            let remaining = digit_max_count - digit_count;
            let available = digits.len() - start;

            // Find the maximum digit we can pick in allowed range
            if let Some((pos, &max)) = digits.iter()
                .enumerate()
                .skip(start)
                .take(available - remaining + 1)
                .max_by_key(|&(i, d)| (d, -(i as isize)))
            {
                combined_bank_joltage = combined_bank_joltage * 10 + max as u64;
                digit_count += 1;
                start = pos + 1;
            } else {
                break;
            }
        }
        max_output_joltage += combined_bank_joltage;
    }
    return Some(max_output_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
