use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split("\n")
        .filter(|instruction| !instruction.is_empty())
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .map(|ranges| {
            let range_split: Vec<u64> = ranges
                .split('-')
                .map(|range| range.parse().unwrap())
                .collect();
            range_split[0]..=range_split[1]
        })
        .collect()
}

pub fn check_if_valid(id: u64) -> u64 {
    let stringified_id = id.to_string();
    let id_length = stringified_id.chars().count();
    let half_length = id_length / 2;
    if !id_length.is_multiple_of(2) {
        return 0;
    }
    let first_half = stringified_id.chars().collect::<Vec<_>>()[..half_length]
        .iter()
        .collect::<String>();
    let second_half = stringified_id.chars().collect::<Vec<_>>()[half_length..]
        .iter()
        .collect::<String>();

    if first_half == second_half { id } else { 0 }
}

pub fn check_if_valid_part2(id: u64) -> u64 {
    let stringified_id = id.to_string();
    let id_length = stringified_id.chars().count();
    let half_length = id_length / 2;

    if id_length >= 2 {
        let first_char = stringified_id.chars().collect::<Vec<_>>()[0];
        if stringified_id.chars().all(|digit| digit == first_char) {
            return id;
        }
    }

    for chunk_count in 2..=half_length {
        let chars: Vec<char> = stringified_id.chars().collect();
        let char_chunks: Vec<String> = chars
            .chunks(chunk_count)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        let first_chunk = &char_chunks[0];
        if char_chunks.iter().skip(1).all(|chunk| chunk == first_chunk) {
            return id;
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges = parse(input);

    let invalid_sum = id_ranges
        .iter()
        .map(|id_range| id_range.clone().map(check_if_valid).sum::<u64>())
        .sum();

    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges = parse(input);

    let invalid_sum = id_ranges
        .iter()
        .map(|id_range| id_range.clone().map(check_if_valid_part2).sum::<u64>())
        .sum();

    Some(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_174_379_265));
    }
}
