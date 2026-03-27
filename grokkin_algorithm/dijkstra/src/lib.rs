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

pub struct RouteRequest<'a, 'b> {
    source: &'b str,
    destination: &'b str,
    target_network: Network<'a>,
}

impl<'a, 'b> RouteRequest<'a, 'b> {
    pub fn new(target_network: Network<'a>, source: &'b str, destination: &'b str) -> Self {
        RouteRequest {
            target_network,
            source,
            destination,
        }
    }
}

pub struct OptimalRouteFinder<'a, 'b> {
    route_request: RouteRequest<'a, 'b>,
    // processing_node: HashMap<&'a str, u32>,
    cost_tracker: HashMap<&'a str, Option<u32>>,
    // does the value need to be Option<&str> Can't it not be just &str??
    route_tracker: HashMap<&'a str, Option<&'a str>>,
    processed_nodes: Vec<&'a str>,
    current_node: Option<&'a str>,
    process_next: Option<&'a str>,
}

impl<'a, 'b> OptimalRouteFinder<'a, 'b> {
    pub fn new(find_route: RouteRequest<'a, 'b>) -> Self {
        OptimalRouteFinder {
            route_request: find_route,
            // processing_node: HashMap::new(),
            cost_tracker: HashMap::new(),
            route_tracker: HashMap::new(),
            processed_nodes: Vec::new(),
            current_node: None,
            process_next: None,
        }
    }

    fn initiate_finder(&mut self)
    where
        'b: 'a,
    {
        let source_neighbors = self
            .route_request
            .target_network
            .get(self.route_request.source)
            .unwrap();

        for (node_name, &cost) in source_neighbors {
            self.cost_tracker.insert(node_name, Some(cost));
            self.route_tracker
                .insert(node_name, Some(self.route_request.source));
        }

        self.current_node = Some(self.route_request.source);
    }

    fn set_next_processing_node(&mut self) {
        // checks current node's neighbors
        // sets 'next_processing_node' to cheapest out of the nieghbors
        // returns `None` when it can't find a neighbor from the current node
        let current_processing_node = self.current_node;
        let current_node_neighbors = self
            .route_request
            .target_network
            .get(current_processing_node.expect("check processing node"))
            .unwrap();

        let cheaper_node = current_node_neighbors
            .iter()
            .min_by(|&(_, acc_cost), &(_, e_cost)| acc_cost.cmp(e_cost))
            .map(|(&node_name, _)| node_name);

        self.current_node = cheaper_node;

        // checks for unprocessed nodes in the Network
        // sets 'next_processing_node' to any first unprocessed node found

        // return 'None' when node can't be found after above two process
        // sets cuurent_node to none when RouteRequest.target_network's keys ==
        // finder.processed_node's content
    }

    fn has_visited(&self, node: &str) -> Option<&str> {
        // self.cost_tracker.iter().any(|(&x, _)| x == node)
        self.cost_tracker
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
        // how do I get the current node's cost?
        // cost should be stored in the cost_tracker since it's a neighbor of processed node
        // self.cost_tracker.get(node).unwrap() +
        // ...where do I get the 'node' passed to .get()?
        //
        // self.processing_node.get("node_to_evaluate").unwrap()
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
pub fn run(network: &Network, source: &str, destination: &str) -> String {
    // helper variables to check whether the program processed every node in the given network
    let all_nodes_in_network = network.keys().copied().collect::<Vec<&str>>();
    let processed_nodes: Vec<&str> = Vec::new();

    println!("in fn_run");

    // initiate
    // sets OptimalRouteFinder.cost_tracker,

    // I need an set up function here finder.initiate_finder(frist_node)

    // OptimalRouteFinder.route_tracker,
    // returns the processing node

    // *** Finder traversing
    // load the node to process
    // let mut processing_node_name: &'a str = finder.process_next.expect();
    //
    // get the next processing node from 'network'
    // finder.set_processing_node(network);
    //
    // check the neighbors of current node
    // -> OptimalRouteFinder
    // .has_visited() (within iterator of current_node)
    // .is_optimal() the heart of this program (if has_visited returns
    // false, then this function does not need to be called)
    // .write_cost_tracker()
    //
    // .write_route_tacker()
    //
    // ==cheapest of the node is found here==
    // *** Finder traversing
    //
    // move to the cheapest out of all neighbors
    // -> node = OptimalRouteFinder.get_cheapest_neighbor();
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
        let cheapest_finder = OptimalRouteFinder::new(make_route_request());

        let source = cheapest_finder.route_request.source;
        let destination = cheapest_finder.route_request.destination;
        assert_eq!(source, "u");
        assert_eq!(destination, "z");

        let source_neighbors = cheapest_finder
            .route_request
            .target_network
            .get("u")
            .expect("check route_request");

        assert_eq!(
            *source_neighbors,
            HashMap::from([("a", 6_u32), ("b", 2_u32)])
        );

        let cost = cheapest_finder.cost_tracker.get("u");
        assert_eq!(cost, None);

        let route = cheapest_finder.route_tracker.get("u");
        assert_eq!(route, None);

        let test_current_node = cheapest_finder.current_node;
        assert_eq!(test_current_node, None);

        let next_node = cheapest_finder.process_next;
        assert_eq!(next_node, None);
    }

    #[test]
    fn sets_first_data_to_fields_current_node_and_cost_tracker_and_route_tracker() {
        let mut finder = make_initiated_finder();

        let test_cost_map = HashMap::from([("a", Some(6)), ("b", Some(2))]);
        let test_route_map = HashMap::from([("a", Some("u")), ("b", Some("u"))]);

        finder.initiate_finder();

        assert_eq!(finder.cost_tracker, test_cost_map);
        assert_eq!(finder.route_tracker, test_route_map);
        assert_eq!(finder.current_node, Some("u"));
    }

    #[test]
    fn sets_current_node_to_cheaper_neighbor_of_current_node() {
        let mut finder = make_initiated_finder();
        finder.set_next_processing_node();

        assert_eq!(finder.current_node, Some("b"));
    }
    #[test]
    fn sets_current_node_to_none_when_no_more_node_to_process() {
        let mut finder = make_finder_that_visited_all_nodes();
        finder.set_next_processing_node();

        assert_eq!(finder.current_node, None);
    }
    /* commenting this test out. For my current network,
     * this case cannot logically occur
    #[test]
    fn sets_current_node_to_non_visited_node_in_network() {}
    */

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
    fn get_network() -> Network<'static> {
        HashMap::from([
            ("u", HashMap::from([("a", 6), ("b", 2)])),
            ("a", HashMap::from([("z", 1)])),
            ("b", HashMap::from([("a", 3), ("z", 5)])),
            ("z", HashMap::new()),
        ])
    }

    fn get_source_and_destination() -> (&'static str, &'static str) {
        ("source", "destination")
    }

    fn make_route_request() -> RouteRequest<'static, 'static> {
        RouteRequest {
            source: ("u"),
            destination: ("z"),
            target_network: get_network(),
        }
    }

    fn make_finder() -> OptimalRouteFinder<'static, 'static> {
        OptimalRouteFinder {
            route_request: RouteRequest {
                source: ("u"),
                destination: ("z"),
                target_network: get_network(),
            },
            cost_tracker: HashMap::from([("a", Some(3)), ("b", Some(6))]),
            route_tracker: HashMap::from([("a", Some("u")), ("b", Some("u"))]),
            processed_nodes: Vec::new(),
            current_node: None,
            process_next: None,
        }
    }

    fn make_initiated_finder() -> OptimalRouteFinder<'static, 'static> {
        let route_request = RouteRequest {
            source: ("u"),
            destination: ("z"),
            target_network: get_network(),
        };
        let mut finder = OptimalRouteFinder::new(route_request);
        finder.initiate_finder();

        finder
    }

    fn get_network_nodes() -> Vec<&'static str> {
        let network = get_network();
        network.keys().copied().collect::<Vec<&str>>()
    }

    fn make_finder_that_visited_all_nodes() -> OptimalRouteFinder<'static, 'static> {
        OptimalRouteFinder {
            route_request: RouteRequest {
                source: "u",
                destination: "z",
                target_network: get_network(),
            },
            cost_tracker: HashMap::from([("a", Some(3)), ("b", Some(6))]),
            route_tracker: HashMap::from([("a", Some("u")), ("b", Some("u"))]),
            processed_nodes: get_network_nodes(),
            current_node: Some("z"),
            process_next: Some("b"),
        }
    }
}
