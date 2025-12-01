advent_of_code::solution!(1);

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn convert_from_char(s: &char) -> Option<Direction> {
        match s {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

pub struct Instruction {
    pub direction: Direction,
    pub value: usize,
}

impl Instruction {
    pub fn new(direction: &char, value: usize) -> Self {
        Instruction {
            direction: Direction::convert_from_char(direction).unwrap(),
            value,
        }
    }

    pub fn get_index_and_zero_count(
        &self,
        _safe_dial_start: usize,
        safe_dial_end: usize,
        current_index: usize,
        part1: bool,
    ) -> (usize, usize) {
        match self.direction {
            Direction::Left => {
                let new_index =
                    (current_index + safe_dial_end - (self.value % safe_dial_end)) % safe_dial_end;
                if part1 {
                    (new_index, if new_index == 0 { 1 } else { 0 })
                } else {
                    let passes = (self.value + safe_dial_end - current_index) / safe_dial_end;
                    if current_index == 0 && passes > 0 {
                        // If we start at zero and move left, we count an extra pass
                        (new_index, passes - 1)
                    } else {
                        (new_index, passes)
                    }
                }
            }
            Direction::Right => {
                let new_index = (current_index + self.value) % safe_dial_end;
                if part1 {
                    (new_index, if new_index == 0 { 1 } else { 0 })
                } else {
                    let passes = (current_index + self.value) / safe_dial_end;
                    (new_index, passes)
                }
            }
        }
    }

    pub fn print(&self) {
        match self.direction {
            Direction::Left => println!("Move Left by {}", self.value),
            Direction::Right => println!("Move Right by {}", self.value),
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .filter(|instruction| !instruction.is_empty())
        .map(|instruction| {
            let instruction_pieces = instruction.trim().chars().collect::<Vec<char>>();
            Instruction::new(
                &instruction_pieces[0],
                instruction_pieces[1..]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse(input);

    let mut current_index = 50;
    let mut zero_counter = 0;
    for instruction in instructions.iter() {
        let (new_index, zero_count) =
            instruction.get_index_and_zero_count(0, 100, current_index, true);
        current_index = new_index;
        zero_counter += zero_count;
    }

    Some(zero_counter as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse(input);

    let mut current_index = 50;
    let mut zero_counter = 0;
    for instruction in instructions.iter() {
        let (new_index, zero_count) =
            instruction.get_index_and_zero_count(0, 100, current_index, false);
        current_index = new_index;
        zero_counter += zero_count;
    }

    Some(zero_counter as u64)
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
