# Graphs

## Description

Implement idiomatic graph data structures (tree, forest, graph...) that are dynamically modifiable during runtime.

## Example

A simple graph structure:

```rust
struct Graph<T> {
    // The graph contains only a Vec of nodes.
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    //  Each node of the graph corresponds to an index.
    index: usize,
    //  The node knows the index of its neighbors.
    adjacent: Vec<u32>,
    // The actual data associated to this node.
    pub data: T,
}
```

A more advanced example: a forest

```rust
// A node of the graph.
pub struct Node<T> {
    // Each node can have at max one parent: multiple roots can exist.
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    // The actual data stored.
    pub data: T,
}

// indexes from the vector used for creating the graph.
pub struct NodeId {
    index: usize,
}

// A memory management arena that ensures that every element of the arena has the same lifetime.
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    // Method to create a new node.
    pub fn new_node(&mut self, data: T) -> NodeId {
        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }
}
```

## Motivation

This pattern should be used when we need data structures that can be modified during runtime.

## Advantages

- Clear and easy to use.
- Reduce the lifetime complexity within the structures.
- Avoid runtime borrow checks.
- Every element within the arena has the same lifetime.
- Multi processing is possible given that parts of a vector can be shared across threads safely.

## Discussion

Region-based memory management is the common method used to deal with graph data structures in Rust.
This avoids to use interior mutability with data structures as follows:

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    previous: Rc<RefCell<Box<Node<T>>>>,
    next: Vec<Rc<RefCell<Box<T>>>>,
    data: T,
    // ...
}
```

which is hard to understand and will also lead into runtime borrow checks.

## See also

- [Region-based memory management](https://en.wikipedia.org/wiki/Region-based_memory_management)
- [Module rustc_data_structures::graph](https://doc.rust-lang.org/1.1.0/rustc_data_structures/graph/)
- [Code example: Arena based tree structure with multithreading support](https://github.com/saschagrunert/indextree)
- [Code example: Graph data structure library](https://github.com/bluss/petgraph)
- [Code example: How to implement complex data structure in Rust](https://github.com/danigm/rust-graph-example)
