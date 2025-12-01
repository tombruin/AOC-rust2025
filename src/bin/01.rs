advent_of_code::solution!(1);


const R: u8 = b'R';
const L: u8 = b'L';

pub fn part_one(input: &str) -> Option<u64> {
    let mut vaultcode: i32 = 50;
    let mut vaultzero: u64 = 0;

    for line in input.lines() {
        let (direction, rest) = line.split_at(1);
        let number: i32 = rest.parse().unwrap();
        match direction.as_bytes()[0] {
            R => vaultcode = (vaultcode + number).rem_euclid(100),
            L => vaultcode = (vaultcode - number).rem_euclid(100),
            _    => {}
        }
        if vaultcode == 0 { vaultzero += 1; }
    }
    return Some(vaultzero);
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
