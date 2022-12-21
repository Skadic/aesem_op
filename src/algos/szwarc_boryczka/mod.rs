use crate::algos::szwarc_boryczka::util::f64_cmp;

use self::{util::{edge_weight, random_path}, harmony_memory::HarmonyMemory};
use super::{
    score_cmp, tsiligiridis_s_algo::SAlgorithm, OrienteeringAlgo, Solution, StandardGraph,
};
use log::{trace, debug};
use petgraph::visit::{EdgeRef, NodeIndexable};
use rand::{
    distributions::{Uniform, WeightedIndex},
    prelude::Distribution,
    Rng, thread_rng, seq::SliceRandom,
};
use std::{collections::HashSet, cmp::Ordering};
use thiserror::Error;

mod twoopt;
mod harmony_memory;
pub mod util;

/// This is a slight generalization of Szwarc and Boryczka's harmony search algorithm.
/// This is not using the center-of-gravity heuristic, since this only works in a metric space.
/// Note that this algorithm still might perform poorly if the graph does not satisfy the triangle equation.
#[derive(Default)]
pub struct SzwarcBoryczka {
    /// The maximum amount of stored solutions in the harmony memory
    harmony_memory_size: usize,
    /// Harmony Memory Consideration Rate
    /// This is the rate by which a value from the harmony memory is chosen.
    /// If a value is not chosen from HM, then it is chosen randomly from the set of permitted values.
    hmcr: f64,
    /// Pitch Adjustment Rate
    /// The rate by which a value chosen from the harmony memory is slightly modified.
    par: f64,
    /// The number of iterations
    iterations: usize,
}

type Graph = petgraph::Graph<f64, f64, petgraph::Undirected, usize>;

#[derive(Clone, Copy, PartialEq, Debug, Error)]
pub enum SzwarcBoryczkaError {
    #[error("value \"{name}\" must be between 0 and 1, found \"{value}\"")]
    InvalidPercentage { value: f64, name: &'static str },
    #[error("harmony memory size must be greater than 0")]
    ZeroHarmonyMemorySize,
}

impl SzwarcBoryczka {
    pub fn new(
        harmony_memory_size: usize,
        hmcr: f64,
        par: f64,
        iterations: usize,
    ) -> Result<Self, SzwarcBoryczkaError> {
        if harmony_memory_size == 0 {
            return Err(SzwarcBoryczkaError::ZeroHarmonyMemorySize);
        }

        if hmcr.is_infinite() || hmcr.is_nan() || hmcr < 0.0 || hmcr > 1.0 {
            return Err(SzwarcBoryczkaError::InvalidPercentage {
                value: hmcr,
                name: "hmcr",
            });
        }

        if par.is_infinite() || par.is_nan() || par < 0.0 || par > 1.0 {
            return Err(SzwarcBoryczkaError::InvalidPercentage {
                value: par,
                name: "par",
            });
        }

        Ok(SzwarcBoryczka {
            harmony_memory_size,
            hmcr,
            par,
            iterations,
        })
    }


    /// Filters the given vector such that it only contains vectors that do not violate the budget if included next in the path.
    ///
    /// # Panics
    ///
    /// Panics if current does not have a connection to all other nodes. This should always be the case though since this algorithm is only for complete graphs.
    fn filter_available_nodes(
        &self,
        remaining_vertices: &mut HashSet<usize>,
        graph: &StandardGraph,
        remaining_budget: f64,
        current: usize,
        end: usize,
    ) {
        let current = graph.from_index(current);
        let end = graph.from_index(end);

        // Only retain nodes which if included would not violate the remaining budget
        remaining_vertices.retain(|&node| {
            let node = graph.from_index(node);
            let e0 = graph
                .edges_connecting(current, node)
                .next()
                .expect("the graph should be complete");
            let e1 = graph
                .edges_connecting(node, end)
                .next()
                .expect("the graph should be complete");

            let w0 = graph[e0.id()];
            let w1 = graph[e1.id()];

            w0 + w1 <= remaining_budget
        });
    }

    fn choose_node(
        &self,
        available_nodes: &HashSet<usize>,
        harmony_memory: &HarmonyMemory,
        new_harmony: &Vec<usize>,
        graph: &StandardGraph,
        rng: &mut impl Rng,
        dist: &Uniform<f64>,
    ) -> usize {
        let r = dist.sample(rng);
        // Check if we want to choose from HM or randomly
        if r < self.hmcr {
            let current = *new_harmony
                .last()
                .expect("at least the start node should be in the new harmony");

            // Generate a random value between 0 and 1 to choose whether we want to modify our choice
            let k = dist.sample(rng);

            // Check if we want to modify our value or not
            if k < self.par {
                // If we only have one node the choice is obvious
                // This early return prevents issues with the probability distribution later
                // It really doesn't like all weights in the distribution being zero
                if available_nodes.len() == 1 {
                    return *available_nodes.iter().next().unwrap();
                }

                // If this is the case, modify our choice slightly. We need some more processing for this though
                // We calculate each node's ranks in the 
                let available_list = available_nodes.iter().copied().collect::<Vec<_>>();
                let mut score_ranks = (0..available_list.len()).collect::<Vec<_>>();
                let mut distance_ranks = score_ranks.clone();

                score_ranks.sort_unstable_by(|&a, &b| {
                    f64_cmp(
                        &graph[graph.from_index(a)],
                        &graph[graph.from_index(b)]
                    )
                });
                distance_ranks.sort_unstable_by(|&a, &b| {
                    f64_cmp(
                        &edge_weight(current, available_list[b], graph).unwrap_or(0.0),
                        &edge_weight(current, available_list[a], graph).unwrap_or(0.0)
                    )
                });

                trace!("Available Nodes: {available_list:?}");
                trace!("Score Ranks: {score_ranks:?}");
                trace!("Distance Ranks: {distance_ranks:?}");

                let heuristic_dist = WeightedIndex::new(
                    (0..available_nodes.len())
                        .map(|i| 0.8 * (available_nodes.len() - i - 1) as f64 + 0.2 * distance_ranks[i] as f64),
                ).expect("at least one weight must be nonzero");

                let choice = heuristic_dist.sample(rng);
                available_list[choice]
            } else {
                harmony_memory.choose_next(current, available_nodes, graph, rng)
            }
        } else {
            // Choose a random value from the available indices
            *available_nodes
                .iter()
                .nth(rng.gen_range(0..available_nodes.len()))
                .unwrap()
        }
    }

}

impl OrienteeringAlgo for SzwarcBoryczka {
    #[allow(unused)]
    fn generate_path(
        &mut self,
        graph: &Graph,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Solution> {
        // Setup for sampling random numbers between 0 and 1
        let mut rng = rand::thread_rng();
        let percent_dist = Uniform::new_inclusive(0.0, 1.0);

        // This stores a limited amount of previously found solutions together with their required score.
        // These are used to "improvise" new solutions
        let mut harmony_memory = HarmonyMemory::generate(graph, self.harmony_memory_size, start, end, max)?;

        let mut iteration = 0;
        let mut iterations_from_last_replacement = 0;

        let start = graph.from_index(start);
        let end = graph.from_index(end);

        let mut available_vertices = HashSet::with_capacity(graph.node_count());

        // Iterate
        for i in 0..self.iterations {
            debug!("------ Iteration {i} --------");
            let mut current_cost = 0.0;
            let mut new_harmony = vec![start.index()];
            // Generate the indices available right now
            available_vertices.clear();
            available_vertices.extend(
                graph
                    .node_indices()
                    .filter(|&idx| idx != start && idx != end)
                    .map(|idx| idx.index()),
            );
            trace!("Initially available nodes: {available_vertices:?}");
            // Only retain nodes that we can still include given our remaining budget
            self.filter_available_nodes(
                &mut available_vertices,
                graph,
                max - current_cost,
                start.index(),
                end.index(),
            );
            trace!("After filtering with remaining budget {}: {available_vertices:?}", max - current_cost);

            while !available_vertices.is_empty() {
                let chosen_node = self.choose_node(
                    &available_vertices,
                    &harmony_memory,
                    &new_harmony,
                    graph,
                    &mut rng,
                    &percent_dist,
                );

                // The index of the node that is now inserted
                let new_node_index = new_harmony.len();
                new_harmony.push(chosen_node);

                // Add the weight of this edge to the current cost
                current_cost += edge_weight(
                    new_harmony[new_node_index - 1],
                    new_harmony[new_node_index],
                    graph,
                )
                .expect("this graph should be complete");
                available_vertices.remove(&chosen_node);
                // Filter out non-available indices again
                self.filter_available_nodes(
                    &mut available_vertices,
                    graph,
                    max - current_cost,
                    new_harmony[new_node_index],
                    end.index(),
                );
            }
            new_harmony.push(end.index());

            let solution = Solution::evaluate(new_harmony, graph);
            // todo!("upgrade the new harmony if better")
            harmony_memory.insert(solution);
        }

        harmony_memory.into_iter().max_by(score_cmp)
    }
}
