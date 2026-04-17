use dijkstra::RouteRequest;
use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

#[test]
fn result_should_contain_the_final_state_of_cost_tracker_after_loop_terminates() {
    let source = "u";
    let destination = "z";

    // create Network
    let network: Network = HashMap::from([
        ("u", HashMap::from([("a", 6), ("b", 2)])),
        ("a", HashMap::from([("z", 1)])),
        ("b", HashMap::from([("a", 3), ("z", 5)])),
        ("z", HashMap::new()),
    ]);

    let route_request = RouteRequest::new(network, source, destination);

    let (cost_tracker, route_tracker) = dijkstra::run(route_request);
    println!("final result: {0:?}", cost_tracker);
    println!("final result: {0:?}", route_tracker);

    assert_eq!(
        cost_tracker,
        HashMap::from([("a", Some(5_u32)), ("b", Some(2)), ("z", Some(6))])
    );
    assert_eq!(
        route_tracker,
        HashMap::from([("a", Some("b")), ("b", Some("u")), ("z", Some("a"))])
    );
}
