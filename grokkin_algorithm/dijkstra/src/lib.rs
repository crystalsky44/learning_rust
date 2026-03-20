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

pub struct CheapestFinder<'a> {
    from_to: (&'a str, &'a str),
    processing_node: HashMap<&'a str, u32>,
    cheapest_cost_tracker: HashMap<&'a str, u32>,
    cheapest_route_tracker: HashMap<&'a str, &'a str>,
    process_next: Option<&'a str>,
}

impl<'a> CheapestFinder<'a> {
    pub fn new(source: &'a str, destination: &'a str) -> Self {
        CheapestFinder {
            from_to: (source, destination),
            processing_node: HashMap::new(),
            cheapest_cost_tracker: HashMap::new(), // I might want this to be CostTracker, so that I can add impl
            cheapest_route_tracker: HashMap::new(),
            process_next: None,
        }
    }

    fn has_visited(&self, node: &str) -> Option<&str> {
        // self.cheapest_cost_tracker.iter().any(|(&x, _)| x == node)
        self.cheapest_cost_tracker
            .iter()
            .find(|&(&tracker_node, _)| tracker_node == node)
            .map(|(&x, _)| x)
    }

    // the finder should already know the node to evaluate
    // 1 need: the cost of node under evaluation in cost_tracker
    // 2 need: the new cost from processing node
    // 3 need: the cost to processing node from the source
    //
    // compare:
    // (cost to processing node from source + cost to the evaluating node
    // from processing node) < the cost in cost tracker
    //
    // returns the cost if the new cost makes the cost cheaper
    // returns None if not

    // calculate the cost to visited node with new cost from the source,
    fn add_current_node_and_new_cost(&self, node_to_evaluate: &str) -> u32 {
        todo!()
    }

    // determines whether the current written cost is cheaper then the new cost to the visited node and returns the cost if cheaper
    fn new_cost_is_cheaper(&self, new_cost: u32) -> Option<u32> {
        todo!()
    }

    /* I don't know if I need this now
    fn get_optimal_cost(&self, node: &HashMap<&'a str, u32>) -> Option<u32> {
    todo!()
    }
    */

    fn write_cost_tracker(&mut self, node_to_write: &str, cost_to_write: u32) {
        todo!()
    }

    // I might need to ask few questions about this
    fn write_route_tracker(&mut self) {}

    fn get_cheapest_neighbor(node: &HashMap<&'a str, u32>) -> Option<&'a str> {
        todo!()
    }
}

// the run function can maybe said as a program's process archietecture
pub fn run(network: &Network /*, source: &str, destination: &str*/) -> String {
    // get all the nodes in the Network
    let all_nodes_in_network = network.keys().copied().collect::<Vec<&str>>();
    let processed_nodes: Vec<&str> = Vec::new();
    println!("in fn_run");

    // initiate
    // sets CheapestFinder.cheapest_cost_tracker,
    // CheapestFinder.cheapest_route_tracker,
    // returns the processing node

    // *** Processor tasks
    // get the next processing node from 'network'
    // -> current_node = network.get(node).unwrap()
    //
    // check the neighbors of current node
    // -> CheapestFinder
    // .has_visited() (within iterator of current_node)
    // .is_optimal() the heart of this program (if has_visited returns
    // false, then this function does not need to be called)
    // .write_cost_tracker()
    //
    // .write_route_tacker()
    //
    // ==cheapest of the node is found here==
    // *** Processor tasks
    //
    // move to the cheapest out of all neighbors
    // -> node = CheapestFinder.get_cheapest_neighbor();
    //
    // repeat untill every node is processed
    // -> while has_visited_all_nodes() {}

    // format the output
    todo!()
}

fn initiate() {}

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
    #[test]
    fn returns_node_name_on_visited_nodes() {
        let cheapest_route_finder = make_finder();

        assert_eq!(cheapest_route_finder.has_visited("a"), Some("a"));
    }
    #[test]
    fn returns_none_on_unvisited_node() {
        let cheapest_route_finder = make_finder();

        assert_eq!(cheapest_route_finder.has_visited("finish"), None);
    }

    // helper functions
    fn make_finder() -> CheapestFinder<'static> {
        CheapestFinder {
            from_to: ("start", "finish"),
            cheapest_cost_tracker: HashMap::from([("a", 3), ("b", 6)]),
            cheapest_route_tracker: HashMap::new(),
            process_next: None,
        }
    }
}
