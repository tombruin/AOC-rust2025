advent_of_code::solution!(1);


const R: u8 = b'R';
const L: u8 = b'L';

pub fn part_one(input: &str) -> Option<u64> {
    let mut currentvaultcode: i32 = 50;
    let mut vaultzerocounter: u64 = 0;

    for line in input.lines() {
        let (direction, rest) = line.split_at(1);
        let number: i32 = rest.parse().unwrap();
        match direction.as_bytes()[0] {
            R => currentvaultcode = (currentvaultcode + number).rem_euclid(100),
            L => currentvaultcode = (currentvaultcode - number).rem_euclid(100),
            _    => {}
        }
        if currentvaultcode == 0 { vaultzerocounter += 1; }
    }
    return Some(vaultzerocounter);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut currentvaultcode: i32 = 50;
    let mut vaultzerocounter: u64 = 0;

    for line in input.lines() {
        let (direction, rest) = line.split_at(1);
        let number: i32 = rest.parse().unwrap();
        //Count all 100 iterations
        vaultzerocounter +=  number.div_euclid(100) as u64;
        //Calculate rest
        let restnumber = number.rem_euclid(100);
        //Calculate delta from Left or Right
        let delta = match direction.as_bytes()[0] {
            R => restnumber,
            L => -restnumber,
            _    => 0,
        };
        let nextvaultcode = currentvaultcode + delta;
        if (nextvaultcode <= 0 && currentvaultcode != 0) || (nextvaultcode >= 100  && currentvaultcode != 0) {
            vaultzerocounter += 1;
        }
        currentvaultcode = nextvaultcode.rem_euclid(100);
    }
    return Some(vaultzerocounter);
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
        assert_eq!(result, Some(6));
    }

}

