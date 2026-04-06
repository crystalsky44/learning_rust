/// below are *Translation* of Dijkstra Algorithm's
/// pseudo code in Grokking Algorithm
use std::collections::HashMap;

use dijkstra::{RouteRequest, run};

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn main() {
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

    let reuslt = run(route_request);
}
