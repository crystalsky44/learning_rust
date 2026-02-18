// dijkstr lib.rs (3rd one)

// the goal of this program (Dijkstra Algorithm) is to calculate the shortest
// path from a given graph

use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

struct RoutingTable {
    cheapest_cost: HashMap<String, u32>,
    cheapest_parent: HashMap<String, String>,
}

impl RoutingTable {
    fn new () -> RoutingTable {
        Self {
            cheapest_cost: HashMap::new(),
            cheapest_parent: HashMap::new(),
        }
    }

    fn register_cost() {
        todo!();
    }

    fn track_route() {
        todo!();
    }

    pub fn get_result() -> String {
        todo!();
    }
}

fn run(graph: &Graph) -> String {
    let keys: Vec<&str> = graph.keys().map(|key| *key).collect();
    let cheapest_pairs = RoutingTable::new();

    let starting_key = get_starting_key(graph);

    todo!()

    /* add to later compiling

    // using while loop because I want control over the keys I want to pass to
    // .register_cost()
    while !keys.is_empty() {
        // extracts the parent node and its child node(s) from the graph
        // get the cost (or costs) of the path(s) from parent node to each child node
        let node_in_progress = graph.get(keys);

        // save the cost(s) of the path(s) if the cost to the child node is the cheapest
        // save the parent and child pair of the cheapest path
        cheapest_pairs.register_cost(parent_child_pair);
    }

    // format the cheapest cost and their parent child pairs into String 
    let result = format_to_result(cheapest_pairs);

    // return a String data containing the information of the cheapest paths
    result

    */
}

fn get_starting_key<'a>(graph: &'a Graph<'a>) -> (&'a str, &'a str) {
    // get a list that lists the every key of the graph
    let nodes: Vec<&str> = graph.keys().map(|key| *key).collect();
    // println!("{nodes:?}");

    // get the child keys using the key from the keys attained from above
    let mut out_neighbor_keys: Vec<&str> = Vec::new();

    // push every child keys into a single list
    // delete the duplicates (or don't push if it already exists in the list)
    let mut finish_key: Option<&str> = None;
    for &node in &nodes {
        let out_neighbors = graph.get(node).expect("out_neighbors");
        if out_neighbors.is_empty() {
            finish_key = Some(node);
        }
        // println!("{out_neighbors:?}");

        for (key, _) in out_neighbors {
            if !out_neighbor_keys.contains(key) {
                out_neighbor_keys.push(*key);
            }
        }
    }
    // println!("list of out neigbors' key: {out_neighbor_keys:?}");

    // compare the list of child keys and the list of parent keys
    // and one element (it should be one element) that's not listed
    // in the child keys will be the starting node
    let mut starting_key: Option<&str> = None;
    for &node in &nodes {
        if !out_neighbor_keys.contains(&node) {
            starting_key = Some(node);
        }
    }

    (starting_key.expect("error"), finish_key.expect("error"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let cheapest_route = RoutingTable::new();
        assert!(cheapest_route.cheapest_cost.is_empty());
    }

    #[test]
    fn get_start_key() {
        let graph: Graph = HashMap::from([
            ("start", HashMap::from([("a", 6), ("b", 2)])),
            ("a", HashMap::from([("finish", 2)])),
            ("b", HashMap::from([("a", 3), ("finish", 7)])),
            ("finish", HashMap::new())
        ]);

        let (starting_key, finish_key) = get_starting_key(&graph);
        assert_eq!("start", starting_key);
        assert_eq!("finish", finish_key);
    }
}
