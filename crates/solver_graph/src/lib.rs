#[derive(Debug)]
struct Index(usize);

#[derive(Debug)]
struct Nodes<T>(Vec<T>);

#[derive(Debug)]
struct Edges<T>(Vec<Edge<T>>);

#[derive(Debug)]
struct Edge<T> {
    from: Index,
    to: Index,
    weights: Weights<T>,
}

#[derive(Debug)]
struct Weights<T>(Vec<T>);

#[derive(Debug)]
struct Graph<T> {
    nodes: Nodes<T>,
    edges: Edges<T>,
}

impl From<Vec<(usize, usize)>> for Graph<usize> {
    fn from(value: Vec<(usize, usize)>) -> Self {
        Self {
            nodes: Nodes(vec![]),
            edges: Edges(
                value
                    .into_iter()
                    .map(|(from, to)| Edge {
                        from: Index(from),
                        to: Index(to),
                        weights: Weights(Vec::with_capacity(4)),
                    })
                    .collect(),
            ),
        }
    }
}

impl<T: Copy + Default> Graph<T> {
    fn new() -> Self {
        Self {
            nodes: Nodes(Vec::new()),
            edges: Edges(Vec::new()),
        }
    }

    fn nodes(&mut self, nodes: Vec<T>) -> &mut Self {
        self.nodes.extend(nodes);
        self
    }

    fn edges(&mut self, edges: Vec<(usize, usize)>) -> &mut Self {
        self.edges.extend(edges);
        self
    }

    // TODO: Implement Iterator
    fn last(&self) -> Option<&T> {
        self.nodes.last()
    }
}

impl<T: Copy + Default> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Nodes<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }

    fn first(&self) -> Option<&T> {
        self.0.first()
    }

    fn last(&self) -> Option<&T> {
        self.0.last()
    }
}

impl<T> Edges<T> {
    fn extend<I: IntoIterator<Item = (usize, usize)>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().map(|(from, to)| Edge {
            from: Index(from),
            to: Index(to),
            weights: Weights(Vec::with_capacity(4)),
        }))
    }

    fn first(&self) -> Option<&Edge<T>> {
        self.0.first()
    }

    fn last(&self) -> Option<&Edge<T>> {
        self.0.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        // The following constructs A -> B -> C; A -> C; C -> A
        let nodes = vec![0, 1, 2, 3];
        let edges: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (0, 2), (2, 0)];
        let mut graph = Graph::new(); // TODO: graph![]
        let last = graph.nodes(nodes).edges(edges).last();
        assert_eq!(Some(&3), last)
    }
}
