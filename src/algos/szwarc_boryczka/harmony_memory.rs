use std::collections::HashSet;

use log::{debug, trace};
use petgraph::visit::NodeIndexable;
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng, Rng};

use crate::algos::{score_cmp, szwarc_boryczka::util::edge_weight, Solution, StandardGraph};

use super::util::random_path;

pub(super) struct HarmonyMemory {
    harmonies: Vec<Solution>,
    harmony_memory_size: usize,
}

impl HarmonyMemory {
    /// This generates a few initial solutions.
    /// The algorithm used to create them is Tsiligirides' S-Algorithm
    pub fn generate(
        graph: &StandardGraph,
        harmony_memory_size: usize,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Self> {
        let mut harmonies: Vec<Solution> = Vec::with_capacity(harmony_memory_size);

        let mut rng = thread_rng();

        let mut fails = 0;
        // Fill the harmony memory with initial solutions
        while harmonies.len() < harmony_memory_size {
            // Try to find a path. If none is found, return
            let Some(solution) = random_path(start, end, max, graph, &mut rng) else {
                fails += 1;
                // We don't want to be stuck in an infinite loop trying to find initial solutions
                if fails > 2 * harmony_memory_size {
                    // We'll give up in this case
                    if harmonies.is_empty() {
                        return None;
                    } else {
                        // If we have already found some solutions then we'll just make do with the ones we have
                        break;
                    }
                }
                continue;
            };
            harmonies.push(solution);
        }

        harmonies.sort_unstable_by(score_cmp);

        Some(HarmonyMemory {
            harmonies,
            harmony_memory_size,
        })
    }

    pub fn insert(&mut self, solution: Solution) {
        let worst_score = self.harmonies[0].score;
        let new_score = solution.score;
        if self.harmonies.len() < self.harmony_memory_size {
            // In this case we still have space in our harmony memory
            let index = self
                .harmonies
                .iter()
                .position(|s| s.score > solution.score)
                .unwrap_or(self.harmonies.len());
            self.harmonies.insert(index, solution);
            debug!("Inserted new harmony with score {new_score}")
        } else if worst_score < solution.score {
            // In this case we want to replace the worst harmony with our new one
            // First find the index where we need to insert the new harmony
            let mut index = 0;
            for i in 0..self.harmony_memory_size {
                if i == self.harmony_memory_size - 1 || self.harmonies[i + 1].score > solution.score
                {
                    index = i;
                    break;
                } else {
                    self.harmonies[i] = std::mem::take(&mut self.harmonies[i + 1]);
                }
            }
            self.harmonies[index] = solution;
            debug!("Inserted new harmony with score {new_score}, improved from {worst_score}")
        } else {
            trace!(
                "Did not insert harmony with score {}: {:?}",
                solution.score, solution.path,
            );
        }
    }

    /// Choose a node from the HM that should follow the given node.
    /// If the current node appears in the HM, any node that follows it in HM could be a viable choice. 
    /// If the current ndoe does *not* appear in HM then it is chosen from the available nodes,
    /// weighted by the expected score in relation to the distance traveled to it
    /// 
    /// # Panics
    ///
    /// Panics if the graph is not complete.
    pub fn choose_next(
        &self,
        current: usize,
        available_nodes: &HashSet<usize>,
        graph: &StandardGraph,
        rng: &mut impl Rng,
    ) -> usize {
        // Generate a map of nodes that appear after new_harmony[j] (last element) in the harmony memory
        // associated with the score of the harmony it comes from. If it appears multiple times, the scores are added together
        let mut possible_next_nodes = self
            .iter()
            .filter_map(|Solution { path, score, .. }| {
                // Find the position of our current node in the harmony and if found return the node after it
                path.iter()
                    .position(|&v| v == current)
                    .and_then(|i| path.get(i + 1).copied())
                    .map(|i| (i, *score))
            })
            // We only consider vertices that are available
            .filter(|(node_id, _)| available_nodes.contains(&node_id))
            .collect::<Vec<_>>();

        if possible_next_nodes.is_empty() {
            // In this case our current node does not appear in the harmony memory at all
            // So we need to choose from the remaining nodes
            // We'll 'score' each node by the quotient of earned score and distance to the current node
            possible_next_nodes.extend(available_nodes.iter().map(|&node| {
                (
                    node,
                    graph[graph.from_index(node)]
                        / edge_weight(current, node, graph).expect("graph should be complete"),
                )
            }));
        }

        // Create a weighted distribution to choose a node from the list
        let weighted_dist = WeightedIndex::new(possible_next_nodes.iter().map(|(_, score)| score))
            .expect("scores should be above zero and the list of scores non-empty");

        let choice = weighted_dist.sample(rng);

        possible_next_nodes[choice].0
    }

    pub fn iter(&self) -> std::slice::Iter<Solution> {
        self.harmonies.iter()
    }
}

impl IntoIterator for HarmonyMemory {
    type Item = Solution;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.harmonies.into_iter()
    }
}
