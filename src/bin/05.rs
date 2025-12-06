advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let ingredient_id_ranges = splitted_input.get(0).unwrap();

    let sum_fresh_products_ranges: Vec<_> = ingredient_id_ranges
        .lines()
        .filter_map(|incredient_idpair| incredient_idpair.split_once('-'))
        .map(|(startid, endid)| {
            (
                startid.parse::<u64>().unwrap(),
                endid.parse::<u64>().unwrap(),
            )
        })
        .map(|ingredient_idrange| ingredient_idrange.0..=ingredient_idrange.1)
        .collect();

    let fresh_ingredient_ids = splitted_input
        .get(1)
        .unwrap()
        .lines()
        .filter_map(|id| {
            let ingredientid = id.parse::<u64>().ok()?;
            if sum_fresh_products_ranges
                .iter()
                .any(|range| range.contains(&ingredientid))
            {
                Some(ingredientid)
            } else {
                None
            }
        })
        .count();

    return Some(fresh_ingredient_ids as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let splitted_input: Vec<&str> = input.split("\n\n").collect();

    let mut ingredient_id_ranges: Vec<(u64, u64)> = splitted_input.get(0).unwrap()
        .lines()
        .filter_map(|pair| pair.split_once('-'))
        .map(|(startid, endid)| {
            (
                startid.parse::<u64>().unwrap(),
                endid.parse::<u64>().unwrap(),
            )
        })
        .collect();

    // Sort on the starting IngredientId
    ingredient_id_ranges.sort_by_key(|&(start, _)| start);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ingredient_id_ranges {
        if let Some((_last_start, last_end)) = merged_ranges.last_mut() {
            if start <= *last_end + 1 {
                //extend the already existing range by the max
                *last_end = (*last_end).max(end);
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    //For every idividual range count how many items are in there
    let total: u64 = merged_ranges.iter().map(|(start_id, end_id)| end_id - start_id + 1).sum();
    return Some(total);
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
        assert_eq!(result, Some(14));
    }
}
