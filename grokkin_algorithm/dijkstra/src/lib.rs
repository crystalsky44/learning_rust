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
use std::collections::HashMap;

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
    cost_tracker: HashMap<&'a str, Option<u32>>,
    // does the value need to be Option<&str> Can't it not be just &str??
    route_tracker: HashMap<&'a str, Option<&'a str>>,
    current_node_name: Option<&'a str>,
    current_node: HashMap<&'a str, u32>,
    processed_nodes: Vec<&'a str>,
}

impl<'a, 'b> OptimalRouteFinder<'a, 'b> {
    pub fn new(find_route: RouteRequest<'a, 'b>) -> Self {
        OptimalRouteFinder {
            route_request: find_route,
            cost_tracker: HashMap::new(),
            route_tracker: HashMap::new(),
            current_node_name: None,
            current_node: HashMap::new(),
            processed_nodes: Vec::new(),
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

        self.current_node_name = Some(self.route_request.source);
        self.current_node = source_neighbors.clone();
    }

    fn next_node_name(&mut self) {
        let cheaper_node = self.current_node
            .iter()
            .min_by(|&(_, acc_cost), &(_, e_cost)| acc_cost.cmp(e_cost))
            .map(|(&node_name, _)| node_name);
        self.current_node_name = cheaper_node;
    }


    // sequentially coupled to fn next_processing_node_name
    fn set_new_current_node(&mut self) {
        let Some(next_node) = 
            self.route_request.target_network.get(self.current_node_name.unwrap()) else
        {
            return;
        };

        self.current_node = next_node.clone();
    }

    fn set_next_processing_node(&mut self) {
        // checks current node's neighbors
        // sets 'next_processing_node' to cheapest out of the nieghbors
        // returns `None` when it can't find a neighbor from the current node
        self.next_node_name();
        self.set_new_current_node();

        //*** below are logics not implemented for this project
        //*** but should be in production code

        // checks for unprocessed nodes in the Network
        // sets 'next_processing_node' to any first unprocessed node found

        // return 'None' when node can't be found after above two process
        // sets cuurent_node to none when RouteRequest.target_network's keys ==
        // finder.processed_node's content
    }

    fn evaluate_path(&mut self) {
        let evaluating_node = &self.current_node;

        let cost_to_current_node = self
            .cost_tracker
            .get(self.current_node_name.expect("node_name"))
            .unwrap()
            .unwrap();

        for (&node_name, &cost) in evaluating_node {
            self.cost_tracker
                .entry(node_name)
                .and_modify(|tracker_cost| {
                    if tracker_cost.unwrap() > cost_to_current_node + cost {
                        *tracker_cost = Some(cost_to_current_node + cost);
                    }
                })
                .or_insert(Some(cost_to_current_node + cost));
        }
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
}

// the run function can maybe said as a program's process archietecture
pub fn run(route_request: RouteRequest) {
    // helper variables to check whether the program processed every node in the given network

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

// compare with the list of input nodes and return true if it has
/*
fn has_visited_all_nodes(network_nodes: &[&str], processed_nodes: &[&str]) -> bool {
    let network = network_nodes.iter().collect::<HashSet<_>>();
    let processed = processed_nodes.iter().collect::<HashSet<_>>();

    network == processed
}
*/

#[cfg(test)]
mod tests {
    use super::*;

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

        let test_current_node = cheapest_finder.current_node_name;
        assert_eq!(test_current_node, None);
    }

    #[test]
    fn sets_first_data_to_fields_current_node_and_cost_tracker_and_route_tracker() {
        let mut finder = make_initiated_finder();

        let test_cost_map = HashMap::from([("a", Some(6)), ("b", Some(2))]);
        let test_route_map = HashMap::from([("a", Some("u")), ("b", Some("u"))]);

        finder.initiate_finder();

        assert_eq!(finder.cost_tracker, test_cost_map);
        assert_eq!(finder.route_tracker, test_route_map);
        assert_eq!(finder.current_node_name, Some("u"));
    }

    #[test]
    fn sets_current_node_to_cheaper_neighbor_of_current_node() {
        let mut finder = make_initiated_finder();
        finder.set_next_processing_node();

        assert_eq!(finder.current_node_name, Some("b"));
        assert_eq!(
            finder.current_node,
            HashMap::from([("a", 3_u32), ("z", 5_u32)])
        );
    }
    #[test]
    fn sets_current_node_to_none_when_no_more_node_to_process() {
        let mut finder = make_finder_that_visited_all_nodes();
        finder.set_next_processing_node();

        assert_eq!(finder.current_node_name, None);
    }
    /* commenting this test out. For my current network,
     * this case cannot logically occur
    #[test]
    fn sets_current_node_to_non_visited_node_in_network() {}
    */

    // tests for evaluate_path
    #[test]
    fn new_entry_to_trackers_when_processing_node_with_only_new_node_name() {
        let mut finder = make_initiated_finder();
        finder.set_next_processing_node();

        finder.evaluate_path();

        assert_eq!(finder.cost_tracker["z"], Some(7_u32));
    }
    #[test]
    fn modify_trackers_when_processing_node_with_visited_node() {
        let mut finder = make_initiated_finder();
        println!("{0:?}", finder.current_node_name);
        println!("{0:?}", finder.cost_tracker);
        finder.set_next_processing_node();
        println!("{0:?}", finder.current_node_name);
        println!("{0:?}", finder.current_node);

        finder.evaluate_path();
        println!("{0:?}", finder.cost_tracker);

        assert_eq!(finder.cost_tracker["a"], Some(5_u32));
    }
    #[test]
    #[ignore]
    fn does_not_modify_trackers_when_processing_node_with_visited_node() {
        let mut finder = make_initiated_finder();
        finder.set_next_processing_node();
        println!("{0:?}", finder.cost_tracker);

        finder.evaluate_path();
        println!("{0:?}", finder.cost_tracker);

        assert_eq!(finder.cost_tracker["a"], Some(5_u32));
        assert_eq!(finder.cost_tracker["f"], Some(7_u32));
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
            cost_tracker: HashMap::from([("a", Some(5)), ("b", Some(2))]),
            route_tracker: HashMap::from([("a", Some("u")), ("b", Some("u"))]),
            current_node_name: Some("b"),
            current_node: HashMap::new(),
            processed_nodes: Vec::new(),
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
            cost_tracker: HashMap::from([("a", Some(3)), ("b", Some(2))]),
            route_tracker: HashMap::from([("a", Some("u")), ("b", Some("u"))]),
            current_node_name: None,
            current_node: HashMap::new(),
            processed_nodes: get_network_nodes(),
        }
    }
}
