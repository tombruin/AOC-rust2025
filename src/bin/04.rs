advent_of_code::solution!(4);

const DIRECTIONS: [(isize, isize); 8] = [
    (1, -1),  // NE
    (1, 0),   // E
    (1, 1),   // SE
    (0, -1),  // N
    (0, 1),   // S
    (-1, -1), // NW
    (-1, 0),  // W
    (-1, 1),  // SW
];

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| if char == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let y_height_max = grid.len() as isize;
    let x_width_max = grid[0].len() as isize;

    let answer = (0..x_width_max)
        .flat_map(|x| (0..y_height_max).map(move |y| (x, y))) //Make itterator for whole grid
        .filter(|&(x, y)| grid[y as usize][x as usize] == 1) //Filter all field with a @/1 and continue
        .filter(|&(x, y)| {
            let neighbors = DIRECTIONS
                .iter()
                .filter(|&&(dx, dy)| {
                    let nx = x + dx;
                    let ny = y + dy;
                    //Make sure to not walk outside of the grid
                    nx >= 0
                        && nx < x_width_max
                        && ny >= 0
                        && ny < y_height_max
                        && grid[ny as usize][nx as usize] == 1
                })
                .count();
            neighbors < 4
        }) //Filter in all diretions and see if there are more then 4 @/1's
        .count();
    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| if char == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let y_height_max = grid.len() as isize;
    let x_width_max = grid[0].len() as isize;
    let mut total_removed: u32 = 0;

    loop {
        // Phase: find all rolls to remove
        let to_remove_rolls: Vec<_> = (0..x_width_max)
            .flat_map(|x| (0..y_height_max).map(move |y| (x, y))) //Make itterator for whole grid
            .filter(|&(x, y)| grid[y as usize][x as usize] == 1) //Filter all field with a @/1 and continue
            .filter(|&(x, y)| {
                let neighbors = DIRECTIONS
                    .iter()
                    .filter(|&&(dx, dy)| {
                        let nx = x + dx;
                        let ny = y + dy;
                        nx >= 0
                            && nx < x_width_max
                            && ny >= 0
                            && ny < y_height_max
                            && grid[ny as usize][nx as usize] == 1
                    })
                    .count();
                neighbors < 4
            })
            .collect(); //Filter in all diretions and see if there are more then 4 @/1's

        // Nothing more to remove. break
        if to_remove_rolls.is_empty() {
            break;
        }
        total_removed += to_remove_rolls.len() as u32;

        // Remove them
        for (x, y) in &to_remove_rolls {
            grid[*y as usize][*x as usize] = 0;
        }
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
