advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn get_area(&self, other_point: &Point) -> u64 {
        (other_point.x.abs_diff(self.x) + 1) * (other_point.y.abs_diff(self.y) + 1)
    }
}

#[derive(Clone, Debug)]
pub struct PointPair {
    pub point1: Point,
    pub point2: Point,
    pub area: u64,
}

impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        (self.point1 == other.point1) && (self.point2 == other.point2)
    }
}

impl Eq for PointPair {}

impl PointPair {
    pub fn new(point1: Point, point2: Point) -> Self {
        PointPair {
            point1,
            point2,
            area: point1.get_area(&point2),
        }
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area
            .partial_cmp(&other.area)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| {
            let split_row = row.split(',');
            let mut row_nums = split_row.map(|num| num.parse().unwrap());
            Point {
                x: row_nums.next().unwrap(),
                y: row_nums.next().unwrap(),
            }
        })
        .collect()
}

pub fn get_pairs(points: &[Point]) -> Vec<PointPair> {
    let mut point_pairs = Vec::new();
    for outer_index in 0..(points.len() - 1) {
        let first_point = points[outer_index];
        for second_point in points.iter().skip(outer_index + 1) {
            point_pairs.push(PointPair::new(first_point, *second_point));
        }
    }
    point_pairs.sort();

    point_pairs
}

pub fn part_one(input: &str) -> Option<u64> {
    let coordinates = parse(input);
    let coordinate_pairs = get_pairs(&coordinates);

    // println!("{:?}", coordinate_pairs);

    Some(coordinate_pairs.last().unwrap().area)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
