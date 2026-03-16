/// below are *Translation* of Dijkstra Algorithm's
/// pseudo code in Grokking Algorithm
use std::collections::HashMap;

use dijkstra::run;

type Network<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn main() {
    let source = "start";
    let destination = "finish";

    // create Network
    let network: Network = HashMap::from([
        (source, HashMap::from([("a", 6), ("b", 2)])),
        ("a", HashMap::from([(destination, 1)])),
        ("b", HashMap::from([("a", 3), (destination, 5)])),
        (destination, HashMap::new()),
    ]);

    let reuslt = run(&network /*, source, destination*/);
    println!("{reuslt}");
}
