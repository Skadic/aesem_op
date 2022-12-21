use std::{cmp::Ordering, vec};

use petgraph::visit::{EdgeRef, NodeIndexable};
use rand::{seq::SliceRandom, Rng};

use crate::algos::{Solution, StandardGraph};

/// Gets the edge weight between the two given nodes.
/// This requires that there is an edge between them
///
/// # Panics
///
/// Panics if there
pub(super) fn edge_weight(from: usize, to: usize, graph: &StandardGraph) -> Option<f64> {
    Some(
        graph[graph
            .edges_connecting(graph.from_index(from), graph.from_index(to))
            .next()?
            .id()],
    )
}

pub fn f64_cmp(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

pub(super) fn random_path(
    start: usize,
    end: usize,
    max: f64,
    graph: &StandardGraph,
    rng: &mut impl Rng,
) -> Option<Solution> {
    let mut path = vec![start];

    if edge_weight(start, end, graph).unwrap() > max {
        return None;
    }

    let mut available_nodes = graph
        .node_indices()
        .map(|idx| idx.index())
        .filter(|&i| i != start && i != end)
        .collect::<Vec<_>>();
    let mut current_cost = 0.0;

    while !available_nodes.is_empty() {
        let choice = rng.gen_range(0..available_nodes.len());
        let node = available_nodes.swap_remove(choice);

        // If the new node fits in our path insert it, otherwise it can never be added (triangle inequality)
        let weight_to_node = edge_weight(*path.last().unwrap(), node, graph).unwrap(); 
        let weight_to_end = edge_weight(node, end, graph).unwrap();
        if current_cost + weight_to_node + weight_to_end <= max {
            path.push(node);
            current_cost += weight_to_node;
        }
    }
    path.push(end);

    Some(Solution::evaluate(path, graph))
}
