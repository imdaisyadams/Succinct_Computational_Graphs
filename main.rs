/// A builder that will be used to create a computational graph.
struct Builder {
    nodes: Vec<Node>,
    constraints: Vec<usize, usize>, //Nodes that are equal
}

/// A node in the computational graph.
#[derive(Clone, Debug)]
struct Node {
    id: usize,
    node_type: NodeType,
    value: Option<u32>,
    left_child: Option<usize>,
    right_child: Option<usize>,
}

#[derive(Clone, Debug)]
enum NodeType {
    Input,
    Constant(u32),
    Add,
    Multiple,
    Hint,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder {
            nodes: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    /// Initializes a node in the graph.
    pub fn init(&mut self, node_type: NodeType, left_child: Option<usize>, right_child: Option<usize>) -> Node {

    }
    
    /// Initializes a node in a graph, set to a constant value.
    pub fn constant(&mut self, value: u32) -> Node {
        todo!()
    }
    
    /// Adds 2 nodes in the graph, returning a new node.
    pub fn add(&mut self, a: Node, b: Node) -> Node {
        todo!()
    }
    
    /// Multiplies 2 nodes in the graph, returning a new node.
    pub fn mul(&mut self, a: Node, b: Node) -> Node {
        todo!()
    }
    
    /// Asserts that 2 nodes are equal.
    pub fn assert_equal(&mut self, a: Node, b: Node) {
        todo!()
    }
    
    /// Fills in all the nodes of the graph based on some inputs.
    pub fn fill_nodes(&mut self, ...) {
        todo!()
    }
    
    /// Given a graph that has `fill_nodes` already called on it
    /// checks that all the constraints hold.
    pub fn check_constraints(&mut self) -> bool {
        todo!()
    }
    
    /// An API for hinting values that allows you to perform operations
    /// like division or computing square roots.
    pub fn hint(...) ... {
        todo!()
    }
}

// Example 1: f(x) = x^2 + x + 5

let builder = Builder::new();
let x = builder.init();
let x_squared = builder.mul(x, x);
let five = builder.constant(5);
let x_squared_plus_5 = builder.add(x_square, five);
let y = builder.add(x_squared_plus_5, x);

builder.fill_nodes(...)
builder.check_constraints(...)


// Example 2: f(a) = (a+1) / 8
//
// function f(a):
//     b = a + 1
//     c = b / 8  
//     return c
    
let builder = Builder::new();
let a = builder.init();
let b = builder.add(a, 1);

// TODO: determine an API for hint where it can depend
// on the computed value of b and a user can specify an
// arbitrary function for c based on b.
let c = builder.hint(...);
let eight = builder.constant(8);
let c_times_8 = builder.mul(c, eight);
builder.assert_equal(b, c_times_8);

builder.fill_nodes(...);
builder.check_constraints(...);


// Example 3: f(x) = sqrt(x+7)
//
// Assume that x+7 is a perfect square (so x = 2 or 9, etc.).

let builder = Builder::new();
let x = builder.init();
let seven = builder.constant(7);
let x_plus_seven = builder.add(x, seven);

// TODO: determine an API for hint where it can depend
// on the computed value of x+7 and a user can specify
// that the value should be the sqrt.
let sqrt_x_plus_7 = builder.hint(...);
let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
builder.assert_equal(computed_sq, x_plus_seven);

builder.fill_nodes(...);
builder.check_constraints(...);