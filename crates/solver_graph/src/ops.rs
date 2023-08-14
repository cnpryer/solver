use std::{collections::HashMap, hash::Hash, ops::Add};

use crate::{
    graph::Graph,
    queue::PriorityQueue,
    small_array::{Reduce, Reducer, SmallArray},
    Position, Value,
};

/// Sort the `Nodes` of the `Graph`.
///
/// ```rust
/// use solve_graph::{Graph, sort};
///
/// let mut graph = Graph::new();
/// let mut graph = sort(&mut graph);
/// ```
pub(crate) fn sort<P: Position, V: Value>(_graph: &mut Graph<V, P>) -> &mut Graph<V, P> {
    unimplemented!()
}

/// Query the shortest path from a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, shortest_path};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let path = find_shortest_path(&graph, 0, 1).unwrap();
/// ```
pub(crate) fn shortest_path<P: Position + Ord + Hash, V: Value + Ord + Add<Output = V>>(
    graph: &Graph<V, P>,
    from: P,
    to: P,
) -> Option<Vec<&V>> {
    let mut weights = HashMap::new();
    let mut prev_nodes: HashMap<P, Option<P>> = HashMap::new();
    // TODO(cnpryer): What capacity do I want for this? Shouldn't need V * E right? What about V + E
    let mut queue = PriorityQueue::with_capacity(graph.nodes().len());

    let start = from;
    weights.insert(start, SmallArray::Empty);
    queue.push((start, SmallArray::Empty));

    while let Some((node, weight)) = queue.pop() {
        if node == to {
            // Found the target node, reconstruct the path
            let mut path = Vec::new();
            let mut current = node;
            while let Some(Some(prev)) = prev_nodes.get(&current) {
                // Node is on path
                path.push(
                    graph
                        .nodes()
                        .get(current.into())
                        .unwrap_or_else(|| panic!("node ({:?})", current)),
                );
                if current == start {
                    break;
                }
                current = *prev;
            }
            path.reverse();
            return Some(path);
        }

        // TODO(cnpryer): Can I implement a cheaper `Copy` for `SmallArray<V>`? Don't want to clone
        if let Some(edges) = graph.edges().get(node) {
            for edge in edges.iter() {
                let to = &edge.to;
                let w = weight.reduce(Reducer::SumArray(
                    edge.weights().unwrap_or(&SmallArray::Empty),
                ));
                if let Some(d) = weights.get(to) {
                    if &w < d {
                        weights.insert(*to, w.clone());
                        prev_nodes.insert(*to, Some(node));
                        queue.push((*to, w.clone()));
                    }
                } else {
                    weights.insert(*to, w.clone());
                    prev_nodes.insert(*to, Some(node));
                    queue.push((*to, w.clone()));
                }
            }
        }
    }

    None // No path found
}

/// Query the longest path from a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, longest_path};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let path = longest_path(&graph, 0, 1).unwrap();
/// ```
pub(crate) fn longest_path<P: Position, V: Value>(
    _graph: &Graph<V, P>,
    _from: usize,
    _to: usize,
) -> Option<Vec<&V>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        graph,
        graph::test_fixtures::{sample_nodes, sample_weighted_edges},
    };

    #[test]
    fn test_shortest_path() {
        let (nodes, edges) = (sample_nodes(), sample_weighted_edges());
        let graph = graph![nodes, edges];
        let path = shortest_path(&graph, 0, 2).unwrap();
        assert_eq!(path, vec![&0, &1, &2]);
    }

    // #[test]
    // fn test_longest_path() {
    //     let (nodes, edges) = (sample_nodes(), sample_edges());
    //     let graph = graph![nodes.clone(), edges.clone()];
    //     let path = longest_path(&graph, 0, 1).unwrap();
    //     assert_eq!(path, vec![&0, &2]);
    // }
}
