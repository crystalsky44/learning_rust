/// below are *Translation* of Dijkstra Algorithm's
/// pseudo code in Grokking Algorithm
use std::collections::HashMap;

use dijkstra::{RouteRequest, run};

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn main() {
    let source = "start";
    let destination = "finish";

    // create Network
    let network: Network = HashMap::from([
        ("start", HashMap::from([("a", 6), ("b", 2)])),
        ("a", HashMap::from([("finish", 1)])),
        ("b", HashMap::from([("a", 3), ("finish", 5)])),
        ("finish", HashMap::new()),
    ]);

    let find_route = RouteRequest::new(&network, source, destination);

    let reuslt = run(find_route);
    println!("{reuslt}");
}
