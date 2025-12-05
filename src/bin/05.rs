use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<u64>) {
    let mut input_split = input.split("\n\n").filter(|row| !row.is_empty());

    let ranges = input_split
        .next()
        .unwrap()
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|range| {
            let mut numbers = range.split("-");
            vec![
                numbers.next().unwrap().to_string().parse::<u64>().unwrap(),
                numbers.next().unwrap().to_string().parse::<u64>().unwrap(),
            ]
        })
        .collect();

    let ingredient_ids = input_split
        .next()
        .unwrap()
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    (ranges, ingredient_ids)
}

pub fn merge_overlapping_ranges(ranges: Vec<Vec<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged_ranges = vec![RangeInclusive::new(ranges[0][0], ranges[0][1])];
    for range in ranges.iter().skip(1) {
        let merged_ranges_len = merged_ranges.len();
        let mut found_in_range = false;
        for (index, merged_range) in merged_ranges.iter_mut().take(merged_ranges_len).enumerate() {
            let mut updated_range = merged_range.clone();
            if updated_range.contains(&range[0]) && !updated_range.contains(&range[1]) {
                updated_range = RangeInclusive::new(*merged_range.start(), range[1]);
                found_in_range = true;
            }
            if !updated_range.contains(&range[0]) && updated_range.contains(&range[1]) {
                updated_range = RangeInclusive::new(range[0], *merged_range.end());
                found_in_range = true;
            }
            if found_in_range {
                merged_ranges[index] = updated_range;
                break;
            }
            if updated_range.contains(&range[0]) && updated_range.contains(&range[1]) {
                found_in_range = true;
                break;
            }
        }
        if !found_in_range {
            merged_ranges.push(RangeInclusive::new(range[0], range[1]));
        }
    }

    merged_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, ids) = parse(input);
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let updated_ranges = merge_overlapping_ranges(ranges);

    Some(
        ids.iter()
            .map(|id| updated_ranges.iter().any(|range| range.contains(id)) as u64)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let updated_ranges = merge_overlapping_ranges(ranges);

    Some(
        updated_ranges
            .iter()
            .map(|range| (range.end() - range.start()) + 1)
            .sum(),
    )
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
