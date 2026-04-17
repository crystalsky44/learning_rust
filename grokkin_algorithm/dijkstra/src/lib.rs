use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

pub struct RouteRequest<'a> {
    source: &'a str,
    target_network: Network<'a>,
}

impl<'a> RouteRequest<'a> {
    pub fn new(target_network: Network<'a>, source: &'a str) -> Self {
        RouteRequest {
            target_network,
            source,
        }
    }
}

pub struct OptimalRouteFinder<'a> {
    route_request: RouteRequest<'a>,
    cost_tracker: HashMap<&'a str, Option<u32>>,
    route_tracker: HashMap<&'a str, Option<&'a str>>,
    current_node_name: Option<&'a str>,
    current_node: HashMap<&'a str, u32>,
    processed_nodes: Vec<&'a str>,
}

impl<'a> OptimalRouteFinder<'a> {
    pub fn new(find_route: RouteRequest<'a>) -> Self {
        OptimalRouteFinder {
            route_request: find_route,
            cost_tracker: HashMap::new(),
            route_tracker: HashMap::new(),
            current_node_name: None,
            current_node: HashMap::new(),
            processed_nodes: Vec::new(),
        }
    }

    fn initiate(&mut self) {
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

    fn push_to_processed_nodes(&mut self) {
        let processed_node_name = self.current_node_name.expect("should_be_there");
        self.processed_nodes.push(processed_node_name);
    }

    fn next_node_name(&mut self) {
        let cheaper_node = self
            .current_node
            .iter()
            .min_by(|&(_, acc_cost), &(_, e_cost)| acc_cost.cmp(e_cost))
            .map(|(&node_name, _)| node_name);

        self.current_node_name = cheaper_node;
    }

    // sequentially coupled to fn next_processing_node_name
    fn set_new_current_node(&mut self) {
        let Some(current_node_name) = self.current_node_name else {
            return;
        };

        let Some(next_node) = self.route_request.target_network.get(current_node_name) else {
            self.current_node = HashMap::new();
            return;
        };

        self.current_node = next_node.clone();
    }

    fn set_next_processing_node(&mut self) {
        if self.current_node_name.is_none() {
            return;
        }

        self.push_to_processed_nodes();
        self.next_node_name();
        self.set_new_current_node();
    }

    fn evaluate_path(&mut self) {
        if self.current_node_name.is_none() {
            return;
        }

        let current_node = &self.current_node;
        let current_node_name = &self.current_node_name.expect("check initiation");
        let cost_tracker = &mut self.cost_tracker;
        let route_tracker = &mut self.route_tracker;
        let mut route_needs_update = false;

        let cost_to_current_node = cost_tracker.get(current_node_name).unwrap().unwrap();

        for (&neighbor_node_name, &cost_to_neighbor) in current_node {
            let cost_to_neighbor_from_source = cost_to_current_node + cost_to_neighbor;
            cost_tracker
                .entry(neighbor_node_name)
                .and_modify(|tracker_cost| {
                    if tracker_cost.unwrap() > cost_to_neighbor_from_source {
                        route_needs_update = true;
                        *tracker_cost = Some(cost_to_neighbor_from_source);
                    }
                })
                .or_insert({
                    route_needs_update = true;
                    Some(cost_to_neighbor_from_source)
                });

            if route_needs_update {
                route_tracker
                    .entry(neighbor_node_name)
                    .and_modify(|parent_name| *parent_name = Some(current_node_name))
                    .or_insert(Some(current_node_name));
            }
        }
    }

    fn has_visited_all_nodes(&self) -> bool {
        let network = self.route_request.target_network.len();
        let processed = self.processed_nodes.len();

        network == processed
    }

    pub fn print_trackers(&self) {
        println!("cost_tracker: {:?}", self.cost_tracker);
        println!("route_tracker: {:?}", self.route_tracker);
    }
}

// the run function can maybe said as a program's process archietecture
pub fn run<'a>(route_request: RouteRequest<'a>) -> OptimalRouteFinder<'a> {
    let mut finder = OptimalRouteFinder::new(route_request);
    finder.initiate();

    let mut all_nodes_visited = finder.has_visited_all_nodes();

    while !all_nodes_visited {
        finder.set_next_processing_node();
        finder.evaluate_path();

        all_nodes_visited = finder.has_visited_all_nodes();
    }

    finder
}

#[cfg(test)]
mod tests {
    use super::*;

    // finder constructor test
    #[test]
    fn new_finder() {
        let cheapest_finder = OptimalRouteFinder::new(make_route_request());

        let source = cheapest_finder.route_request.source;
        assert_eq!(source, "u");

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

    // finder initiation test
    #[test]
    fn sets_first_data_to_fields_current_node_and_cost_tracker_and_route_tracker() {
        let mut finder = make_initiated_finder();

        let test_cost_map = HashMap::from([("a", Some(6)), ("b", Some(2))]);
        let test_route_map = HashMap::from([("a", Some("u")), ("b", Some("u"))]);

        finder.initiate();

        assert_eq!(finder.cost_tracker, test_cost_map);
        assert_eq!(finder.route_tracker, test_route_map);
        assert_eq!(finder.current_node_name, Some("u"));
    }

    // update `current_node` tests
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

    // tests for evaluate_path
    #[test]
    fn new_entry_to_trackers_when_processing_node_with_only_new_node_name() {
        let mut finder = make_initiated_finder();
        finder.set_next_processing_node();

        finder.evaluate_path();

        assert_eq!(finder.cost_tracker["z"], Some(7_u32));
        assert_eq!(finder.route_tracker["b"], Some("u"));
    }
    #[test]
    fn trackers_after_processing_node_b_with_visited_node() {
        let mut finder = make_initiated_finder();
        println!("{0:?}", finder.current_node_name);
        println!("{0:?}", finder.cost_tracker);
        println!("{0:?}", finder.route_tracker);

        finder.set_next_processing_node();
        println!("{0:?}", finder.current_node_name);
        println!("{0:?}", finder.current_node);

        finder.evaluate_path();
        println!("{0:?}", finder.cost_tracker);
        println!("{0:?}", finder.route_tracker);

        assert_eq!(finder.cost_tracker["a"], Some(5_u32));
        assert_eq!(finder.route_tracker["a"], Some("b"));
    }

    // tests for OptimalRouteFinder::has_visited_all_nodes();
    #[test]
    fn returns_true_when_all_nodes_have_been_visited() {
        let finder = make_finder_that_visited_all_nodes();
        assert!(finder.has_visited_all_nodes());
    }
    #[test]
    fn returns_false_when_there_are_not_yet_visited_nodes() {
        let finder = make_initiated_finder();
        assert!(!finder.has_visited_all_nodes());
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

    fn make_route_request() -> RouteRequest<'static> {
        RouteRequest {
            source: ("u"),
            target_network: get_network(),
        }
    }

    fn make_initiated_finder() -> OptimalRouteFinder<'static> {
        let route_request = RouteRequest {
            source: ("u"),
            target_network: get_network(),
        };
        let mut finder = OptimalRouteFinder::new(route_request);
        finder.initiate();

        finder
    }

    fn make_finder_that_visited_all_nodes() -> OptimalRouteFinder<'static> {
        OptimalRouteFinder {
            route_request: RouteRequest {
                source: "u",
                target_network: get_network(),
            },
            cost_tracker: HashMap::from([("a", Some(5)), ("b", Some(2)), ("z", Some(6))]),
            route_tracker: HashMap::from([("a", Some("b")), ("b", Some("u")), ("z", Some("a"))]),
            current_node_name: None,
            current_node: HashMap::new(),
            processed_nodes: vec!["u", "b", "a", "z"],
        }
    }
}
