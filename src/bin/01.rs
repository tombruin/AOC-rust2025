advent_of_code::solution!(1);


const R: &str = "R";
const L: &str = "L";

pub fn part_one(input: &str) -> Option<u64> {
    let mut vaultcode: i32 = 50;
    let mut vaultzero: u64 = 0;

    for line in input.lines() {
        match line.split_at(1){
            (R, number) => {
                vaultcode = (vaultcode + number.parse::<i32>().unwrap()).rem_euclid(100);
            }
            (L, number) => {
                vaultcode = (vaultcode - number.parse::<i32>().unwrap()).rem_euclid(100);
            }
            _ => {}
        };
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
