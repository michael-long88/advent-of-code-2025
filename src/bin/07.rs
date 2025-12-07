advent_of_code::solution!(7);

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect())
        .collect()
}

pub fn update_beams(splitter_index: usize, tachyon_beams: &mut [u64], part2: bool) -> u64 {
    let left_index = splitter_index - 1;
    let right_index = splitter_index + 1;
    if tachyon_beams[splitter_index] != 0 {
        if part2 {
            tachyon_beams[left_index] += tachyon_beams[splitter_index];
            tachyon_beams[right_index] += tachyon_beams[splitter_index];
            tachyon_beams[splitter_index] = 0;
        } else {
            tachyon_beams[splitter_index] = 0;
            tachyon_beams[left_index] = (tachyon_beams[left_index] + 1).min(1);
            tachyon_beams[right_index] = (tachyon_beams[right_index] + 1).min(1);
        }
        return 1;
    }

    0
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse(input);
    let mut beams: Vec<u64> = vec![0; rows.len()];
    let mut beam_split_count = 0;
    let start_index = rows
        .first()
        .unwrap()
        .iter()
        .position(|char| char == &'S')
        .unwrap();
    beams[start_index] = 1;

    rows.iter().skip(2).for_each(|row| {
        row.iter().enumerate().for_each(|(index, char)| {
            if char == &'^' {
                beam_split_count += update_beams(index, &mut beams, false);
            }
        });
    });

    Some(beam_split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = parse(input);
    let mut beams: Vec<u64> = vec![0; rows.len()];
    let start_index = rows
        .first()
        .unwrap()
        .iter()
        .position(|char| char == &'S')
        .unwrap();
    beams[start_index] = 1;

    rows.iter().skip(2).for_each(|row| {
        row.iter().enumerate().for_each(|(index, char)| {
            if char == &'^' {
                _ = update_beams(index, &mut beams, true);
            }
        });
    });

    Some(beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
