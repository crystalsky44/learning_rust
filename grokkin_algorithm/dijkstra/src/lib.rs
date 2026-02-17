// dijkstra lib.rs (3rd one)

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
        todo!("the method is still under construction");
    }

    fn track_route() {
        todo!("the method is still under construction");
    }

    pub fn get_result() -> String {
        todo!("the method is still under construction");
    }
}

fn run(graph: &Graph) -> String {
    let mut keys: Vec<&str> = graph.keys().map(|key| *key).collect();
    let cheapest_pairs = RoutingTable::new();

    todo!()

    /* add to later compiling

    // using while loop because I want control over the keys I want to pass to
    // .register_cost()
    while !keys.is_empty() {
        // extracts the parent node and its child node(s) from the graph
        // get the cost (or costs) of the path(s) from parent node to each child node
        let parent_child_pair = extract_parent(graph);

        // save the cost(s) of the path(s) if the cost to the child node is the cheapest
        // save the parent and child pair of the cheapest path
        cheapest_pairs.save_cheapest_pair(parent_child_pair);
    }

    // format the cheapest cost and their parent child pairs into String 
    let result = format_to_result(cheapest_pairs);

    // return a String data containing the information of the cheapest paths
    result

    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let cheapest_route = RoutingTable::new();
        assert!(cheapest_route.cheapest_cost.is_empty());
    }
}
