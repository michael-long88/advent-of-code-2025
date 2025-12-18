use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::delimited,
};

advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub indicator_check: String,
    pub wiring_schematics: Vec<Vec<usize>>,
    pub joltage_requirements: Vec<u16>,
}

impl Machine {
    pub fn get_fewest_button_presses(&self) -> usize {
        let num_lights = self.indicator_check.chars().count();
        for combo_count in 1..=self.wiring_schematics.len() {
            let combinations: Vec<Vec<&Vec<usize>>> = self
                .wiring_schematics
                .iter()
                .combinations_with_replacement(combo_count)
                .collect();
            // if num_lights == 4 {
            //     println!("Combinations: {:?}", combinations);
            // }
            for combo in combinations.iter() {
                let mut bits: Vec<u8> = vec![0; num_lights];
                for button in combo {
                    for index in button.iter() {
                        bits[*index] ^= 1;
                    }
                }
                if convert_bits_to_string(&bits) == self.indicator_check {
                    // println!("Match with {} button presses.", combo_count);
                    return combo_count;
                }
            }
        }
        self.wiring_schematics.len()
    }
}

fn convert_bits_to_string(bits: &[u8]) -> String {
    bits.iter()
        .map(|bit| if bit == &1 { '#' } else { '.' })
        .collect()
}

fn parse_indicator_check(input: &str) -> IResult<&str, String> {
    delimited(char('['), take_while1(|c| c == '.' || c == '#'), char(']'))
        .map(|s: &str| s.to_string())
        .parse(input)
}

fn parse_usize_list(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), map_res(digit1, |s: &str| s.parse::<usize>())),
        char(')'),
    )
    .parse(input)
}

fn parse_wiring_schematics(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let (input, _) = multispace0(input)?;
    separated_list1(multispace0, parse_usize_list).parse(input)
}

fn parse_joltage_requirements(input: &str) -> IResult<&str, Vec<u16>> {
    delimited(
        char('{'),
        separated_list1(char(','), map_res(digit1, |s: &str| s.parse::<u16>())),
        char('}'),
    )
    .parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, indicator_check) = parse_indicator_check(input)?;
    let (input, _) = multispace0(input)?;
    let (input, wiring_schematics) = parse_wiring_schematics(input)?;
    let (input, _) = multispace0(input)?;
    let (input, joltage_requirements) = parse_joltage_requirements(input)?;

    Ok((
        input,
        Machine {
            indicator_check,
            wiring_schematics,
            joltage_requirements,
        },
    ))
}

pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter_map(|line| parse_machine(line).ok().map(|(_, m)| m))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines: Vec<Machine> = parse(input);

    Some(
        machines
            .iter()
            .map(|machine| machine.get_fewest_button_presses())
            .sum(),
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
