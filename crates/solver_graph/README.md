# solver-graph

`Graph` can be used for operations on `Nodes` and `Edges`.

```rust
struct Graph<T, U: Into<usize>> {
    nodes: Nodes<T>,
    edges: Edges<U, T>,
}
```

Constructing a `Graph` requires using the `graph!` macro to index `nodes` and `edges`.

```rust
use solve_graph::{graph, nodes, edges};

// `Nodes` are all indexed by their input position
let nodes = nodes(vec![0, 1, 2]);

// `Edges` are optional and indexed by their `Node`'s position
let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);

// Create a `Graph` indexed by `Nodes` with `Edges`
let graph = graph![nodes, edges];
``` 
