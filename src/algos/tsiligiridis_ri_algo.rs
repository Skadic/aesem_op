use std::{any::Any, collections::HashSet, hash::Hash, time::Duration};

use petgraph::{
    adj::{EdgeIndex, NodeIndex},
    visit::{EdgeIndexable, IntoNodeReferences},
};

use super::{szwarc_boryczka::util::f64_cmp, OrienteeringAlgoAdapter, Solution, StandardGraph};

pub struct RIAlgorithm;

impl OrienteeringAlgoAdapter for RIAlgorithm {
    fn adapt_path(
        &mut self,
        graph: &super::StandardGraph,
        mut solution: super::Solution,
        max: f64,
    ) -> super::Solution {
        dbg!(&solution);

        for _ in 0..10 {
            reorder(&mut solution, graph);
            augment_path(&mut solution, graph, max);
        }
        Solution::evaluate(solution.path, graph)
    }
}

/// Given a position in the path, return the position once in the order where just first and second are swapped,
/// and once where in addition the nodes between first and seconds are reversed.
///
/// Arguments:
///
/// * `i`: the index in the original path
/// * `first`: the lower position in the original path to be swapped with the second
/// * `second`: the higher position in the original path to be swapped with the first
fn to_reordered_positions(i: usize, first: usize, second: usize) -> (usize, usize) {
    match i {
        _ if i == first => (second, second),
        _ if i == second => (first, first),
        _ if first < i && i < second => (i, second - (i - first)),
        _ => (i, i),
    }
}

/// It tries to find a better path by swapping two nodes in the path.
///
/// Arguments:
///
/// * `solution`: The previous solution that is to be reordered.
/// * `graph`: The graph we're working with.
fn reorder(solution: &mut Solution, graph: &super::StandardGraph) {
    let path = &mut solution.path;
    let cost = &mut solution.cost;

    for first in 1..path.len() - 1 {
        for second in first + 1..path.len() - 1 {
            let mut forward_cost = 0.0;
            let mut backward_cost = 0.0;
            let mut last_forward = path[0];
            let mut last_backward = path[0];

            // Calculate the costs for the path where first and second are swapped
            // (and possibly the nodes inbetween reversed)
            for i in 1..path.len() {
                let (i_forward, i_backward) = to_reordered_positions(i, first, second);
                let (current_forward, current_backward) = (path[i_forward], path[i_backward]);
                //println!("{i} => {i_backward}");
                let forward_edge = graph
                    .edges_connecting(last_forward.into(), current_forward.into())
                    .next()
                    .expect("graph should be complete");
                let backward_edge = graph
                    .edges_connecting(last_backward.into(), current_backward.into())
                    .next()
                    .expect("graph should be complete");
                forward_cost += forward_edge.weight();
                backward_cost += backward_edge.weight();
                last_forward = current_forward;
                last_backward = current_backward;
            }

            // Reorder Nodes
            if forward_cost < *cost && backward_cost < *cost {
                if forward_cost < backward_cost {
                    path.swap(first, second);
                    *cost = forward_cost;
                } else {
                    path[first..=second].reverse();
                    *cost = backward_cost;
                }
            } else if forward_cost < *cost {
                path.swap(first, second);
                *cost = forward_cost;
            } else if backward_cost < *cost {
                path[first..=second].reverse();
                *cost = backward_cost;
            }
        }
    }
}

/// It finds the best insertion for each unvisited node, and then inserts the best one
///
/// Arguments:
///
/// * `solution`: The previous solution to improve on
/// * `graph`: The graph we're working with
/// * `max_cost`: The maximum cost of the solution
///
fn augment_path(solution: &mut Solution, graph: &super::StandardGraph, max_cost: f64) {
    let path = &mut solution.path;
    let path_score = &mut solution.score;
    let path_cost = &mut solution.cost;

    let mut visited = path.iter().copied().collect::<HashSet<_>>();
    
    // The best lowest-cost insertion for any node into the path
    let Some((node, index, insertion_cost, score)) = graph
        .node_references()
        // Filter out nodes with score of basically zero
        .filter(|&(_, &score)| score > 0.0)
        // Filter out visited nodes
        .filter(|(node, _)| !visited.contains(&node.index()))
        // Find the best insertion for each unvisited node
        .map(|(node, &score)| {
            path.windows(2)
                .enumerate()
                .map(|(i, path_nodes)| {
                    // Get the weights of the relevant edges 
                    let before_weight = graph
                        .edges_connecting(path_nodes[0].into(), node)
                        .next()
                        .expect("graph should be complete")
                        .weight();
                    let after_weight = graph
                        .edges_connecting(node, path_nodes[1].into())
                        .next()
                        .expect("graph should be complete")
                        .weight();
                    let between_weight = graph
                        .edges_connecting(path_nodes[0].into(), path_nodes[1].into())
                        .next()
                        .expect("graph should be complete")
                        .weight();

                    (
                        node,
                        i + 1,
                        before_weight + after_weight - between_weight,
                        score,
                    )
                })
                .min_by(|l, r| f64_cmp(&l.2, &r.2))
                .unwrap()
        })
        .filter(|&(_, _, insertion_cost, _)| *path_cost + insertion_cost <= max_cost)
        .min_by(|(_, _, l_cost, l_score), (_, _, r_cost, r_score)| match f64_cmp(l_cost, r_cost) {
        std::cmp::Ordering::Equal => f64_cmp(l_score, r_score).reverse(),
        ord => ord
    })
         else {
        return
    };

    if *path_cost + insertion_cost > max_cost {
        return;
    }

    path.insert(index, node.index());
    *path_cost += insertion_cost;
    *path_score += score;
}
