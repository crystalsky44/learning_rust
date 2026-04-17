use dijkstra::RouteRequest;
use std::collections::HashMap;

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn main() {
    let source = "u";

    // create Network
    let network: Network = HashMap::from([
        ("u", HashMap::from([("a", 6), ("b", 2)])),
        ("a", HashMap::from([("z", 1)])),
        ("b", HashMap::from([("a", 3), ("z", 5)])),
        ("z", HashMap::new()),
    ]);

    let route_request = RouteRequest::new(network, source);

    let result = dijkstra::run(route_request);
    result.print_trackers();
}
