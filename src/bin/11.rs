use std::collections::HashMap;
use std::collections::hash_map::RandomState;

use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use petgraph::{algo, prelude::*};

advent_of_code::solution!(11);

pub fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .filter(|row| !row.is_empty())
        .flat_map(|row| {
            let mut nodes = row.split_whitespace();
            let first_node = nodes.next().unwrap().replace(":", "");
            let mut edges: Vec<(String, String)> = Vec::new();
            for node in nodes {
                edges.push((first_node.to_string(), node.to_string()));
            }
            edges
        })
        .collect()
}

fn count_paths_between(graph: &DiGraph<String, ()>, from: NodeIndex, to: NodeIndex) -> u64 {
    // paths[node] = number of paths from `from` to `node`
    let mut paths: HashMap<NodeIndex, u64> = HashMap::new();
    paths.insert(from, 1);

    // Process in topological order
    let topo = toposort(&graph, None).unwrap();

    // Find where `from` appears in topo order and start there
    let start_idx = topo.iter().position(|&n| n == from).unwrap();

    for &node in &topo[start_idx..] {
        let current_count = *paths.get(&node).unwrap_or(&0);
        if current_count == 0 {
            continue;
        }

        for neighbor in graph.neighbors(node) {
            *paths.entry(neighbor).or_insert(0) += current_count;
        }
    }

    *paths.get(&to).unwrap_or(&0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let edges = parse(input);
    let mut node_lookup = HashMap::<String, NodeIndex>::new();
    let mut graph = DiGraph::<String, ()>::new();

    for edge in &edges {
        let from = *node_lookup
            .entry(edge.0.clone())
            .or_insert_with(|| graph.add_node(edge.0.clone()));
        let to = *node_lookup
            .entry(edge.1.clone())
            .or_insert_with(|| graph.add_node(edge.1.clone()));
        graph.add_edge(from, to, ());
    }

    let start_node = *node_lookup.get("you").unwrap();
    let end_node = *node_lookup.get("out").unwrap();

    let paths =
        algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, start_node, end_node, 0, None)
            .collect::<Vec<_>>();

    Some(paths.len())
}

pub fn part_two(input: &str) -> Option<u64> {
    let edges = parse(input);
    let mut node_lookup = HashMap::<String, NodeIndex>::new();
    let mut graph = DiGraph::<String, ()>::new();

    for edge in &edges {
        let from = *node_lookup
            .entry(edge.0.clone())
            .or_insert_with(|| graph.add_node(edge.0.clone()));
        let to = *node_lookup
            .entry(edge.1.clone())
            .or_insert_with(|| graph.add_node(edge.1.clone()));
        graph.add_edge(from, to, ());
    }

    let start_node = *node_lookup.get("svr").unwrap();
    let end_node = *node_lookup.get("out").unwrap();
    let dac_node = *node_lookup.get("dac").unwrap();
    let fft_node = *node_lookup.get("fft").unwrap();

    let count_1 = count_paths_between(&graph, start_node, dac_node)
        * count_paths_between(&graph, dac_node, fft_node)
        * count_paths_between(&graph, fft_node, end_node);

    let count_2 = count_paths_between(&graph, start_node, fft_node)
        * count_paths_between(&graph, fft_node, dac_node)
        * count_paths_between(&graph, dac_node, end_node);

    Some(count_1 + count_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
