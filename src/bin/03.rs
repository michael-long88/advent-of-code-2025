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

// I could probably modify get_highest_joltage to use the same logic but I'm honestly too tired too right now
pub fn get_highest_joltage_part2(bank: &str) -> u64 {
    let batteries = bank.chars();

    let mut on_batteries: Vec<char> = Vec::new();
    let battery_count = batteries.clone().count();
    let mut window_size = battery_count - 11;
    let mut current_index = 0;

    while on_batteries.len() < 12 {
        let mut window = &bank[current_index..];

        if window.chars().count() == 12 - on_batteries.len() {
            for char in window.chars() {
                on_batteries.push(char);
            }
            break;
        }

        if (current_index + window_size <= battery_count) && on_batteries.len() < 11 {
            window = &bank[current_index..current_index + window_size];
        }

        let max_element = window.chars().max().unwrap();
        let max_element_pos = window.chars().position(|num| num == max_element).unwrap();
        window_size -= max_element_pos;
        current_index += max_element_pos + 1;
        on_batteries.push(max_element);
    }

    on_batteries.iter().collect::<String>().parse().unwrap()
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

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks = parse(input);

    Some(
        battery_banks
            .into_iter()
            .map(get_highest_joltage_part2)
            .sum::<u64>(),
    )
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
        assert_eq!(result, Some(3_121_910_778_619));
    }
}
