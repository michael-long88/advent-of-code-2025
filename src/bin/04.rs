advent_of_code::solution!(4);

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect())
        .collect()
}

pub fn get_adjacent_count(rows: &Vec<Vec<char>>, row_index: isize, col_index: isize) -> u8 {
    let mut roll_count = 0;
    let total_rows = rows.len();
    let total_cols = rows[0].len();
    for row_modifier in -1..=1 {
        for col_modifier in -1..=1 {
            let row_check_index = row_index + row_modifier;
            let col_check_index = col_index + col_modifier;
            if row_check_index < 0
                || col_check_index < 0
                || row_check_index >= total_rows as isize
                || col_check_index >= total_cols as isize
                || (row_modifier == 0 && col_modifier == 0)
            {
                continue;
            }
            roll_count += (rows[row_check_index as usize][col_check_index as usize] == '@') as u8;
        }
    }

    roll_count
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse(input);
    let total_rows = rows.len();
    let total_cols = rows[0].len();
    let mut accessible_roll_count = 0;

    for row_index in 0..total_rows {
        for col_index in 0..total_cols {
            if rows[row_index][col_index] == '.' {
                continue;
            }
            accessible_roll_count +=
                if get_adjacent_count(&rows, row_index as isize, col_index as isize) < 4 {
                    1
                } else {
                    0
                }
        }
    }

    Some(accessible_roll_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows = parse(input);
    let total_rows = rows.len();
    let total_cols = rows[0].len();
    let mut removable_roll_count = 0;
    let mut removed_roll_count = 1;

    while removed_roll_count != 0 {
        let mut removable_roll_indices = Vec::new();
        removed_roll_count = 0;
        for row_index in 0..total_rows {
            for col_index in 0..total_cols {
                if rows[row_index][col_index] == '.' {
                    continue;
                }
                if get_adjacent_count(&rows, row_index as isize, col_index as isize) < 4 {
                    removable_roll_count += 1;
                    removed_roll_count += 1;
                    removable_roll_indices.push((row_index, col_index));
                }
            }
        }
        for index in removable_roll_indices {
            rows[index.0][index.1] = '.';
        }
    }

    Some(removable_roll_count)
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
