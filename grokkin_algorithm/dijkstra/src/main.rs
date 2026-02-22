/// below are *Translation* of Dijkstra Algorithm's
/// pseudo code in Grokking Algorithm
use std::collections::HashMap;

type Graph = HashMap<&str, HashMap<&str, u32>>;

fn main() {
    // create graph
    let graph: Graph = HashMap::from([
        ("start", HashMap::from([("a", 6), ("b", 2)])),
        ("a", HashMap::from([("finish", 1)])),
        ("b", HashMap::from([("a", 3), ("finish", 5)])),
        ("finish", HashMap::new())
    ]);

    // repeat infinity -> I don't know if I can include this in Rust
    todo!("Do I need to replace infinity = float('inf')??");

    // create a hash table to record the cost from the start
    // to the current node
    let mut cost_table = HashMap::from([
        ("a", 6), 
        ("b", 2),
        ("finish", infinity)
        todo!("Above line needs Rust specific translation");
    ]);

    // create another hash table to store the parents
    let mut parent_table = HashMap::from([
        ("a", "start"),
        ("b", "start"),
        ("finish", HashMap::new())
    ]);

    // create an array to keep track of all the processed nodes
    // ** this line was what made me to stuck for a month **
    let mut processed_nodes: Vec<&str> = Vec::new();

    // returns the lowest_
}
