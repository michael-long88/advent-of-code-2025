advent_of_code::solution!(6);

pub enum Operation {
    Add,
    Mult,
}

impl Operation {
    pub fn convert_from_char(s: &char) -> Option<Operation> {
        match s {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Mult),
            _ => None,
        }
    }
}

pub struct ProblemSet {
    pub operation: Operation,
    pub numbers: Vec<u64>,
}

impl ProblemSet {
    pub fn new(operation: &char, numbers: Vec<u64>) -> Self {
        ProblemSet {
            operation: Operation::convert_from_char(operation).unwrap(),
            numbers,
        }
    }

    pub fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Mult => self.numbers.iter().product(),
        }
    }
}

pub fn parse(input: &str) -> Vec<ProblemSet> {
    let rows: Vec<&str> = input
        .split("\n")
        .filter(|instruction| !instruction.is_empty())
        .collect();

    let total_rows = rows.len();

    let number_rows: Vec<Vec<u64>> = rows
        .iter()
        .take(total_rows - 1)
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let digit_count = number_rows[0].len();

    let operators: Vec<char> = rows
        .last()
        .unwrap()
        .split_whitespace()
        .map(|operator| operator.parse().unwrap())
        .collect();

    (0..digit_count)
        .map(|index| {
            let numbers: Vec<u64> = number_rows.iter().map(|row| row[index]).collect();
            let operator = operators.get(index).unwrap();

            ProblemSet::new(operator, numbers)
        })
        .collect()
}

pub fn parse_part2(input: &str) -> Vec<ProblemSet> {
    let rows: Vec<&str> = input
        .split("\n")
        .filter(|instruction| !instruction.is_empty())
        .collect();
    let total_rows = rows.len();
    let operator_row: Vec<char> = rows.last().unwrap().chars().collect();
    let row_length = operator_row.len();
    let number_rows: Vec<Vec<char>> = rows
        .iter()
        .take(total_rows - 1)
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let column_start_indices: Vec<usize> = operator_row
        .iter()
        .enumerate()
        .filter(|(_, value)| *value == &'+' || *value == &'*')
        .map(|(index, _)| index)
        .collect();

    let operators: Vec<char> = column_start_indices
        .iter()
        .map(|index| operator_row[*index])
        .collect();

    let total_columns = column_start_indices.len();
    (0..total_columns)
        .map(|problem_set_col_index| {
            let start_index = column_start_indices[problem_set_col_index];
            let mut end_index = row_length;
            if problem_set_col_index != total_columns - 1 {
                end_index = column_start_indices[problem_set_col_index + 1] - 1
            };

            let numbers: Vec<u64> = (start_index..end_index)
                .rev()
                .map(|col_index| {
                    let mut number = "".to_string();
                    for row in number_rows.clone() {
                        number.push(row[col_index]);
                    }
                    number.trim().parse::<u64>().unwrap()
                })
                .collect();

            ProblemSet::new(&operators[problem_set_col_index], numbers)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .map(|problem_set| problem_set.solve())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_part2(input)
            .iter()
            .map(|problem_set| problem_set.solve())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_277_556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3_263_827));
    }
}
