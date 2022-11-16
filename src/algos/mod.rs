use petgraph::{Graph, adj::NodeIndex};

pub mod tsiligiridi_s_algo;

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

pub trait OrienteeringAlgo<W, C, Dir, Idx> {
    type PathType;
    fn generate_path(&mut self, graph: &Graph<W, C, Dir, Idx>, start: usize, end: usize, max: C) -> Option<Self::PathType>;

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
