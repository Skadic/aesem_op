use std::cmp::Ordering;

use petgraph::{
    visit::{EdgeIndexable, EdgeRef},
    Graph, Undirected,
};

pub mod szwarc_boryczka;
#[allow(unused)]
pub mod tsiligirides_s_algo;

/*
pub struct AlgoChain<F: OrienteeringAlgo<W, C, Dir, Idx>, S: OrienteeringAlgoAdapter<W, C, Dir, Idx, F::PathType>, W, C, Dir, Idx> {
    algo: F,
    ada: S,
}

impl<W, C, Dir, Idx, F: OrienteeringAlgo<W, C, Dir, Idx>, S: OrienteeringAlgoAdapter<W, C, Dir, Idx, F::PathType>> OrienteeringAlgo<W, C, Dir, Idx> for AlgoChain<F, S, W, C, Dir, Idx> {
    type PathType = S::PathType;
}
*/
//self.algo.generate_path(graph, start, end).map(|p| self.ada.adapt_path(graph, p))

type StandardGraph = Graph<f64, f64, Undirected, usize>;

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
            .flat_map(|nodes| graph.edges_connecting(nodes[0].into(), nodes[1].into()).next())
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

    /*fn chain<Ada: OrienteeringAlgoAdapter<W, C, Dir, Idx, >>(self, ada: Ada) -> AlgoChain<Self, Ada>
    where
        Self: Sized,
    {
        AlgoChain { algo: self, ada }
    }*/
}
/*
pub trait OrienteeringAlgoAdapter<W, C, Dir, Idx, InPath> {
    type PathType;
    fn adapt_path(
        &mut self,
        graph: &Graph<W, C, Dir, Idx>,
        path: InPath,
    ) -> Self::PathType;
}*/
