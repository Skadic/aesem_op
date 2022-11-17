use std::cmp::Ordering;
use std::fmt::Debug;

use petgraph::{
    adj::NodeIndex,
    visit::{EdgeRef, NodeIndexable, NodeRef},
    Graph, Undirected,
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

use super::OrienteeringAlgo;

#[derive(Clone, Copy, Debug, Default)]
pub struct SAlgorithm {
    power_factor: f64,
    num_considered: usize,
}

impl SAlgorithm {
    pub fn new(power_factor: f64, num_considered: usize) -> Self {
        Self {
            power_factor,
            num_considered,
        }
    }
}

impl OrienteeringAlgo for SAlgorithm {
    type PathType = Vec<NodeIndex<usize>>;

    fn generate_path(
        &mut self,
        graph: &Graph<f64, f64, Undirected, usize>,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Self::PathType> {
        let n = graph.node_count();

        let end_node = graph.from_index(end);

        // Initializazion
        let mut current = start;
        let mut path = vec![start];
        let mut current_cost = 0.0;
        let mut taken = vec![false; n];
        taken[start] = true;

        let mut rng = thread_rng();

        while current != end {
            let current_node = graph.from_index(current);
            // We want to calculate how desirable each node is to go to
            let desirabilities = {
                let mut temp = graph
                    .edges(current_node)
                    // We want to exclude all nodes already taken and don't want to allow moving to the end node yet
                    .filter(|neigh| !taken[neigh.target().index()] && neigh.target() != end_node)
                    // Check that we can take the edge to the end node from the next node without exceeding the max len
                    .filter(|neigh| {
                        let edge_from_next_to_end = graph
                            .edges_connecting(neigh.target().id(), end_node)
                            .next()
                            .expect(
                                format!(
                                    "node {} does not have an edge to the end node",
                                    neigh.target().id().index()
                                )
                                .as_str(),
                            )
                            .id();
                        let to_next_weight = graph[neigh.id()];
                        let to_end_weight = graph[edge_from_next_to_end];
                        current_cost + to_next_weight + to_end_weight <= max
                    })
                    // get the node id, its score and the cost of the edge leading to it
                    .map(|neigh| (neigh.target(), (&graph[neigh.target()], &graph[neigh.id()])))
                    // Calculate the unnormalized desirability
                    .map(|(node_id, (&score, &cost))| {
                        (node_id, (score / cost).powf(self.power_factor), cost)
                    })
                    .collect::<Vec<_>>();

                // Sort descending
                temp.sort_unstable_by(|(_, a, _), (_, b, _)| {
                    if a < b {
                        return Ordering::Greater;
                    } else if a > b {
                        return Ordering::Less;
                    }
                    Ordering::Equal
                });

                temp.truncate(self.num_considered);

                temp
            };

            if desirabilities.is_empty() {
                if current_cost
                    + graph[graph
                        .edges_connecting(current_node, end_node)
                        .next()
                        .unwrap()
                        .id()]
                    <= max
                {
                    path.push(end);
                }
                return Some(path);
            }

            let dist = WeightedIndex::new(
                desirabilities
                    .iter()
                    .map(|(_, desirability, _)| desirability),
            )
            .expect("no weight should be below 0 and the weights shouldn't be all 0");

            //println!("desirabilities: {desirabilities:?}");
            //println!("distribution: {dist:?}");

            let choice = dist.sample(&mut rng);
            //println!("looking at {num_considered} out of {} elements: chosen elem {choice}", desirabilities.len());

            current = desirabilities[choice].0.index();
            path.push(current);
            taken[current] = true;
            current_cost = current_cost + desirabilities[choice].2;
        }

        Some(path)
    }
}
