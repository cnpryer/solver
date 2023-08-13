mod graph;
mod helpers;
mod ops;
mod small_array;

trait Value: Copy + Default + PartialOrd + Ord + Default {}
impl<V: Copy + Default + PartialOrd + Ord + Default> Value for V {}

trait Position: Copy + Into<usize> {}
impl<P: Copy + Into<usize>> Position for P {}

#[cfg(test)]
mod test_fixtures {
    use crate::{
        graph::{Edges, Nodes},
        helpers::{edge, edges, nodes, weighted_edge},
    };

    pub(crate) fn sample_nodes() -> Nodes<i32> {
        nodes(vec![0, 0, 0, 0])
    }

    pub(crate) fn sample_edges() -> Edges<usize, i32> {
        edges(vec![
            Some(vec![edge(0, 1), edge(0, 2)]),
            Some(vec![edge(1, 2)]),
            Some(vec![edge(2, 0)]),
            None,
        ])
    }

    pub(crate) fn sample_weighted_edges() -> Edges<usize, i32> {
        edges(vec![
            Some(vec![
                weighted_edge(0, 1, vec![1]),
                weighted_edge(0, 2, vec![100]),
            ]),
            Some(vec![weighted_edge(1, 2, vec![1])]),
            Some(vec![weighted_edge(2, 0, vec![2])]),
            None,
        ])
    }
}
