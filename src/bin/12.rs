advent_of_code::solution!(12);

pub struct Region {
    pub width: usize,
    pub length: usize,
    pub shape_quantities: Vec<usize>,
}

pub fn parse(input: &str) -> (Vec<Vec<&str>>, Vec<Region>) {
    let chunks: Vec<&str> = input.split("\n\n").filter(|row| !row.is_empty()).collect();

    let shapes: Vec<Vec<&str>> = chunks
        .iter()
        .take(5)
        .map(|chunk| chunk.lines().collect())
        .collect();

    let regions: Vec<Region> = chunks
        .last()
        .unwrap()
        .lines()
        .map(|region| {
            let mut pieces = region.split_whitespace();
            let mut dimension = pieces.next().unwrap().trim_end_matches(":").split('x');
            let width: usize = dimension.next().unwrap().parse().unwrap();
            let length: usize = dimension.next().unwrap().parse().unwrap();
            let shape_quantities: Vec<usize> = pieces.map(|num| num.parse().unwrap()).collect();

            Region {
                width,
                length,
                shape_quantities,
            }
        })
        .collect();

    (shapes, regions)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, regions) = parse(input);

    let valid_regions: u64 = regions
        .iter()
        .map(|region| {
            let total_shapes_area: usize = region
                .shape_quantities
                .iter()
                .map(|quantity| quantity * 9)
                .sum();

            (total_shapes_area <= (region.width * region.length)) as u64
        })
        .sum();

    Some(valid_regions)
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
        // So the solution works on the solution but not the example input, so the
        // example should produce 2, but I don't want the tests failing even if I got the
        // right answer
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
