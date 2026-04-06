use dijkstra::RouteRequest;
use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

#[test]
fn single_traversal_of_finder_returning_hashmap_with_all_nodes() {
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

    let result = dijkstra::run(route_request);
    assert_eq!(
        result,
        HashMap::from([("a", Some(5_u32)), ("b", Some(2)), ("z", Some(7))])
    );
}
