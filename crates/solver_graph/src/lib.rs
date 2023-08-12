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

impl<T: Copy + Default> Graph<T> {
    fn new() -> Self {
        Self {
            nodes: Nodes(Vec::new()),
            edges: Edges(Vec::new()),
        }
    }

    fn nodes(&self) -> &Nodes<T> {
        &self.nodes
    }

    fn edges(&self) -> &Edges<T> {
        &self.edges
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

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn first(&self) -> Option<&T> {
        self.0.first()
    }

    fn last(&self) -> Option<&T> {
        self.0.last()
    }
}

impl<T> From<&(usize, usize)> for Edge<T> {
    fn from(value: &(usize, usize)) -> Self {
        Self {
            from: Index(value.0),
            to: Index(value.1),
            weights: Weights(Vec::with_capacity(4)),
        }
    }
}

impl<T> Edge<T> {
    fn tuple(&self) -> (usize, usize) {
        (self.from.0, self.to.0)
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

macro_rules! graph {
    ($nodes:expr, $edges:expr) => {{
        Graph {
            nodes: Nodes($nodes),
            edges: Edges(
                $edges
                    .into_iter()
                    .map(|(from, to)| Edge {
                        from: Index(from),
                        to: Index(to),
                        weights: Weights(Vec::with_capacity(4)),
                    })
                    .collect(),
            ),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        // The following constructs A -> B -> C; A -> C; C -> A
        let nodes = vec![0, 1, 2, 3];
        let edges: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (0, 2), (2, 0)];
        let graph = graph![nodes.clone(), edges.clone()];
        assert_eq!(nodes.last(), graph.nodes().last());
        assert_eq!(edges.last(), graph.edges.last().map(|e| e.tuple()).as_ref());
    }
}
