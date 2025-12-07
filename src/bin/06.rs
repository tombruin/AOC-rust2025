advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut rows: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .filter(|row: &Vec<String>| !row.is_empty())
        .collect();

    let symbols = rows.pop().unwrap();

    // Number of columns
    let number_of_columns = rows[0].len();
    let mut matrix: Vec<Vec<u64>> = vec![Vec::new(); number_of_columns];

    // Fill columns directly
    for row in rows {
        for (j, val) in row.into_iter().enumerate() {
            if let Ok(num) = val.parse::<u64>() {
                matrix[j].push(num);
            }
        }
    }

    let mut total: u64 = 0;
    for colum in symbols.iter().enumerate()
    {
        let column_output: u64 = match colum.1.as_bytes()[0] {
            b'+' => matrix[colum.0].iter().sum::<u64>(),
            b'*' => matrix[colum.0].iter().product::<u64>(),
            _    => 0,
        };
        total += column_output as u64;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let max_width = rows.iter().map(|line| line.len()).max().unwrap_or(0);
    let symbols: Vec<_> = rows.pop().unwrap().into_iter().filter(|&c| c != ' ').collect();

    let mut results: Vec<Vec<u64>> = Vec::new();
    let mut current_group: Vec<u64> = Vec::new();

    for col in 0..max_width {
        let mut col_digits = String::new();
        for row in &rows {
            if row[col] != ' ' {
                col_digits.push(row[col]);
            }
        }
        if !col_digits.is_empty() {
            // parse digits into a number and add to current group
            let val = col_digits.parse::<u64>().unwrap();
            current_group.push(val);
        } else {
            // column was empty means end of collomn of numbers. so push
            results.push(current_group);
            current_group = Vec::new();
        }
    }
    // Push the last group if still open
    if !current_group.is_empty() {
        results.push(current_group);
    }

    let mut total: u64 = 0;
    for colum in symbols.iter().enumerate()
    {
        let column_output: u64 = match colum.1 {
            '+' => results[colum.0].iter().sum::<u64>(),
            '*' => results[colum.0].iter().product::<u64>(),
            _    => 0,
        };
        total += column_output as u64;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
