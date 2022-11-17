use std::cmp::Ordering;

use petgraph::visit::NodeIndexable;

use super::{tsiligirides_s_algo::SAlgorithm, OrienteeringAlgo};

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

impl SzwarcBoryczka {

    /// This generates a few initial solutions
    /// The algorithm used to create them is Tsiligirides' S-Algorithm
    fn generate_harmony_memory(
        &self,
        graph: &Graph,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Vec<(f64, Vec<usize>)>> {
        let mut harmony_memory: Vec<(f64, Vec<usize>)> =
            Vec::with_capacity(self.harmony_memory_size);

        let mut fails = 0;
        // Fill the harmony memory with initial solutions
        while harmony_memory.len() < self.harmony_memory_size {
            // Try to find a path. If none is found, return
            let Some(path) = SAlgorithm::new(0.5, 16).generate_path(graph, start, end, max) else {
                fails += 1;
                // We don't want to be stuck in an infinite loop trying to find initial solutions
                if fails > 2 * self.harmony_memory_size {
                    // We'll give up in this case
                    if harmony_memory.is_empty() {
                        return None;
                    } else {
                        // If we have already found some solutions then we'll just make do with the ones we have
                        break;
                    }
                }
                continue;
            };
            let score = path
                .iter()
                .map(|&node_id| graph[graph.from_index(node_id)])
                .sum();
            harmony_memory.push((score, path));
        }

        harmony_memory.sort_unstable_by(tuple_comp);

        Some(harmony_memory)
    }
}

impl OrienteeringAlgo for SzwarcBoryczka {
    type PathType = Vec<usize>;

    #[allow(unused)]
    fn generate_path(
        &mut self,
        graph: &Graph,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Self::PathType> {
        // This stores a limited amount of previously found solutions together with their required score.
        // These are used to "improvise" new solutions
        let mut harmony_memory = self.generate_harmony_memory(graph, start, end, max)?;

        let mut iteration = 0;
        let mut iterations_from_last_replacement = 0;

        for _ in 0..self.iterations {
            let mut new_harmony = vec![start];
            let mut j = 1;
            let mut available_vertices = vec![true; graph.node_count()];
        }

        todo!()
    }
}

fn tuple_comp<O: PartialOrd, T>((a, _): &(O, T), (b, _): &(O, T)) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}