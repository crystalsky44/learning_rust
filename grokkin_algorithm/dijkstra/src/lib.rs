// Calculating the cheapest path in a network given the source node and
// destination node
//
// input: network information, source node and destination node
// output: 1 the cost of cheapest path, 2 the route of the cheapest path
//
// constraint: only use HashMap to store nodes and cost, use dijkstra algorithm
//
// dijkstra algorithm:
// 1. look for neighbor nodes from the source node
// 2. find the neighbor which is cheapest to go to
// 3. do step 1 and 2 with the neighbor you went
// 4. repeat until you finish examining from every node
//
// data:
// 1. to keep track if it went through all the nodes Vec<String>
// 2. to store the information of cheapest route <HashMap<child, parent>>
// 3. to sotre the cost of 2 <u32>
use std::collections::{HashMap, HashSet};

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

struct CheapestFinder<'a> {
    from_to: (&'a str, &'a str),
    cheapest_cost_tracker: HashMap<&'a str, u32>,
    cheapest_route_tracker: HashMap<&'a str, &'a str>,
    process_next: Option<&'a str>,
}

impl<'a> CheapestFinder<'a> {
    fn new(source: &'a str, destination: &'a str) -> Self {
        CheapestFinder {
            from_to: (source, destination),
            cheapest_cost_tracker: HashMap::new(),
            cheapest_route_tracker: HashMap::new(),
            process_next: None,
        }
    }

    fn find_cheapest_neighbor(node: &HashMap<&'a str, u32>) -> Option<&'a str> {
        node.iter().min_by_key
    }

    fn store_to_tracker() {}

    fn next_node() {}
}

// the run function can maybe said as a program's process archietecture
pub fn run(network: &Network /*, source: &str, destination: &str*/) -> String {
    // get all the nodes in the Network
    let all_nodes_in_network = network.keys().copied().collect::<Vec<&str>>();
    let processed_nodes: Vec<&str> = Vec::new();
    println!("in fn_run");

    // *** Processor tasks
    // get the starting node to ignite the process
    // -> node = network.get("start").unwrap()
    //
    // check the neighbors of source node
    // -> CheapestFinder.find_cheapest_neighbor(node);
    //
    // ==cheapest of the node is found here==
    //
    // store their information
    // *** Processor tasks
    //
    // move to the cheapest out of all neighbors
    // -> node = CheapestFinder.next_node();
    //
    // repeat untill every node is processed
    // -> while has_visited_all_nodes() {}

    // format the output
    todo!()
}

/*
fn get_node<'a>(network: &'a Network, node: &str) -> Option<&'a HashMap<&'a str, u32>> {
    network.get(node)
}
*/

// compare with the list of input nodes and return true if it has
fn has_visited_all_nodes(network_nodes: &[&str], processed_nodes: &[&str]) -> bool {
    let network = network_nodes.iter().collect::<HashSet<_>>();
    let processed = processed_nodes.iter().collect::<HashSet<_>>();

    network == processed
}

fn format_output() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visited_all_nodes() {
        // case 1: first loop (only the source)
        let network_nodes = ["one", "two", "three"];
        let processed_nodes = [];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(!checker);
        // case 2: in middle of the loop (source and others)
        let network_nodes = ["one", "two", "three", "four"];
        let processed_nodes = ["two", "one"];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(!checker);

        // case 3: processed all nodes (returns true)
        let network_nodes = ["one", "two", "three", "four"];
        let processed_nodes = ["two", "one", "four", "three"];

        let checker = has_visited_all_nodes(&network_nodes, &processed_nodes);

        assert!(checker);
    }
    #[test]
    fn new_finder() {
        let cheapest_finder = CheapestFinder::new("start", "finish");

        let (from, to) = cheapest_finder.from_to;
        assert_eq!(from, "start");
        assert_eq!(to, "finish");

        let cost = cheapest_finder.cheapest_cost_tracker.get("start");
        assert_eq!(cost, None);

        let route = cheapest_finder.cheapest_route_tracker.get("start");
        assert_eq!(route, None);

        let next_node = cheapest_finder.process_next;
        assert_eq!(next_node, None);
    }
}
