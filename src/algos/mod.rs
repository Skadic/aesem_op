use std::cmp::Ordering;

use petgraph::{
    visit::{EdgeIndexable, EdgeRef},
    Graph, Undirected,
};

#[allow(unused)]
pub mod szwarc_boryczka;
#[allow(unused)]
pub mod tsiligiridis_s_algo;

#[allow(unused)]
pub mod tsiligiridis_ri_algo;

pub type StandardGraph = Graph<f64, f64, Undirected, usize>;

pub struct AlgoChain<F: OrienteeringAlgo, S: OrienteeringAlgoAdapter> {
    algo: F,
    ada: S,
}

impl<F: OrienteeringAlgo, S: OrienteeringAlgoAdapter> OrienteeringAlgo for AlgoChain<F, S> {
    fn generate_path(
        &mut self,
        graph: &StandardGraph,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Solution> {
        self.algo
            .generate_path(graph, start, end, max)
            .map(|path| self.ada.adapt_path(graph, path, max))
    }
}


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Solution {
    pub path: Vec<usize>,
    pub score: f64,
    pub cost: f64,
}

fn score_cmp(a: &Solution, b: &Solution) -> Ordering {
    a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal)
}

impl Solution {
    fn evaluate(path: Vec<usize>, graph: &StandardGraph) -> Self {
        let score = path
            .iter()
            .map(|&id| graph.from_index(id))
            .map(|id| graph[id])
            .sum();
        let cost = path
            .windows(2)
            .flat_map(|nodes| {
                graph
                    .edges_connecting(nodes[0].into(), nodes[1].into())
                    .next()
            })
            .map(|edge| graph[edge.id()])
            .sum();

        Solution { path, score, cost }
    }
}

pub trait OrienteeringAlgo {
    fn generate_path(
        &mut self,
        graph: &StandardGraph,
        start: usize,
        end: usize,
        max: f64,
    ) -> Option<Solution>;

    fn chain<Ada: OrienteeringAlgoAdapter>(self, ada: Ada) -> AlgoChain<Self, Ada>
    where
        Self: Sized,
    {
        AlgoChain { algo: self, ada }
    }
}

pub trait OrienteeringAlgoAdapter {
    fn adapt_path(&mut self, graph: &StandardGraph, solution: Solution, max: f64) -> Solution;
}
