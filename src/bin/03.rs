advent_of_code::solution!(3);

pub fn parse(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .filter(|instruction| !instruction.is_empty())
        .collect()
}

pub fn get_highest_joltage(bank: &str) -> u32 {
    let mut max_num = 10;
    'tens_loop: for tens_digit in (1..=9).rev() {
        if let Some(tens_pos) = bank.find(&format!("{tens_digit}")) {
            let after_tens_pos = &bank[tens_pos + 1..];

            if let Some(max_digit) = after_tens_pos.chars().max() {
                max_num = format!("{}{}", tens_digit, max_digit)
                    .parse::<u32>()
                    .unwrap();

                break 'tens_loop;
            }
        }
    }

    max_num
}

pub fn part_one(input: &str) -> Option<u32> {
    let battery_banks = parse(input);

    Some(
        battery_banks
            .into_iter()
            .map(get_highest_joltage)
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
