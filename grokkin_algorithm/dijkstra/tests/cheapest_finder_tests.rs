use std::collections::HashMap;
use dijkstra::CheapestFinder;

#[test]
fn test_has_visited() {
    let cheapest_router_finder = CheapestFinder {
        from_to: ("start", "finish"),
        cheapest_cost_tracker: HashMap::from([("a", 3), ("b", 6)]),
        cheapest_route_tracker: HashMap::new(),
        process_next: None,
    };

    let test_node = HashMap::from([("a", 3), ("finsih", 6)]);

    let mut true_case = false;
    let mut false_case = true;

    for (&node, _cost) in test_node.iter() {
        if cheapest_router_finder.has_visited(node) {
            true_case = true;
        } else {
            false_case = false;
        }
    }

    assert!(true_case);
    assert!(!false_case);
}
