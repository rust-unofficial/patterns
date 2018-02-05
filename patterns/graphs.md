# Graphs

## Description

Implement a graph data structure that is dynamically modifiable during runtime.


## Example

```rust
// A node of the graph.
pub struct Node<T> {
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

```
struct Node<T> {
    previous: Rc<RefCell<Box<Node<T>>>>,
    next: Vec<Rc<RefCell<Box<T>>>>,
    data: T,
    // ...
}
```

which is hard to understand and will also lead into runtime borrow checks.


## See also

https://en.wikipedia.org/wiki/Region-based_memory_management

https://github.com/saschagrunert/indextree

https://doc.rust-lang.org/1.1.0/rustc_data_structures/graph/

https://github.com/bluss/petgraph

https://github.com/danigm/rust-graph-example
