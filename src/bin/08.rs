use std::collections::HashSet;

advent_of_code::solution!(8);

#[cfg(test)]
const ITERATIONS: usize = 10;

#[cfg(not(test))]
const ITERATIONS: usize = 1_000;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn get_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .partial_cmp(&other.x)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Clone, Debug)]
pub struct PointPair {
    pub point1: Point,
    pub point2: Point,
    pub distance: i64,
}

impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        (self.point1 == other.point1) && (self.point2 == other.point2)
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Eq for PointPair {}

#[derive(Clone, Debug)]
pub struct Graph {
    pub points: HashSet<Point>,
}

impl Graph {
    pub fn add_point_pair(&mut self, point_pair: &PointPair) -> bool {
        if self.points.contains(&point_pair.point1) && !self.points.contains(&point_pair.point2) {
            self.points.insert(point_pair.point2);
            return true;
        } else if !self.points.contains(&point_pair.point1)
            && self.points.contains(&point_pair.point2)
        {
            self.points.insert(point_pair.point1);
            return true;
        } else if self.points.contains(&point_pair.point1)
            && self.points.contains(&point_pair.point2)
        {
            return true;
        }

        false
    }

    pub fn mergeable(&self, other_graph: &Graph) -> bool {
        for other_point in other_graph.points.iter() {
            if self.points.contains(other_point) {
                return true;
            }
        }
        false
    }

    pub fn merge(&mut self, other_graph: &Graph) {
        self.points.extend(other_graph.points.iter());
    }
}

pub fn merge_graphs(graphs: &mut Vec<Graph>) {
    let mut merged = true;
    while merged {
        merged = false;
        let graph_count = graphs.len();
        'outer_loop: for outer_index in 0..graph_count - 1 {
            for inner_index in outer_index + 1..graph_count {
                if graphs[outer_index].mergeable(&graphs[inner_index]) {
                    let inner_loop_graph = graphs[inner_index].clone();
                    graphs[outer_index].merge(&inner_loop_graph);
                    graphs.remove(inner_index);
                    merged = true;
                    break 'outer_loop;
                }
            }
        }
    }
}

pub fn get_pairs(points: &[Point]) -> Vec<PointPair> {
    let mut point_pairs = Vec::new();
    for outer_index in 0..(points.len() - 1) {
        let first_point = points[outer_index];
        for second_point in points.iter().skip(outer_index + 1) {
            let distance = first_point.get_distance(second_point);
            point_pairs.push(PointPair {
                point1: first_point,
                point2: *second_point,
                distance,
            });
        }
    }
    point_pairs.sort();

    point_pairs
}

pub fn parse(input: &str) -> Vec<Point> {
    input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| {
            let points: Vec<i64> = row.split(',').map(|point| point.parse().unwrap()).collect();
            Point {
                x: points[0],
                y: points[1],
                z: points[2],
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = parse(input);
    let point_pairs = get_pairs(&points);
    let mut graphs: Vec<Graph> = Vec::new();

    for pair in point_pairs.iter().take(ITERATIONS) {
        let mut added = false;
        for graph in graphs.iter_mut() {
            added = graph.add_point_pair(pair);
            if added {
                break;
            }
        }
        if !added {
            graphs.push(Graph {
                points: HashSet::from([pair.point1, pair.point2]),
            });
        }
    }

    merge_graphs(&mut graphs);

    let mut graph_sizes: Vec<usize> = graphs.iter().map(|graph| graph.points.len()).collect();
    graph_sizes.sort();

    let result = graph_sizes.iter().rev().take(3).product();

    Some(result)
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25_272));
    }
}
