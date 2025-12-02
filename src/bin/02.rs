advent_of_code::solution!(2);

pub fn has_repeated_sequence(productid: &str) -> bool {
    let len = productid.len();
    // Check equal length else cannot be sequence
    if len % 2 != 0 {
        return false;
    }
    let mid = len / 2;
    let (left, right) = productid.split_at(mid);
    left == right
}

pub fn part_one(input: &str) -> Option<u64> {

    let sum_invalid_productids: u64 = input.trim()
        .split(',')
        .filter_map(|product_idpair| product_idpair.split_once('-'))
        .map(|(startid, endid)| (startid.parse::<u64>().unwrap(), endid.parse::<u64>().unwrap()))
        .flat_map(|product_idrange| (product_idrange.0..=product_idrange.1).filter(|&n| has_repeated_sequence(&n.to_string())))
        .sum();

    return Some(sum_invalid_productids);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
