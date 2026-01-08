#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};

use grokkin_algorithm::{get_neighbors, add_neighbors_to_que};
// use grokkin_algorithm::{get_neighbors};

fn main() {
   let result = run();
   println!("{result}");
}

fn run() -> String {
    let neighbors_network = HashMap::from([
        ("you", Some(vec!["alice", "bob", "claire"])),
        ("bob", Some(vec!["anuj", "peggy"])),
        ("alice", Some(vec!["peggy"])),
        ("claire", Some(vec!["thom", "jonny"])),
        ("anuj", None),
        ("peggy", None),
        ("thom", None),
        ("jonny", None)
    ]);

    let mut neighbor_deque: VecDeque<&str> = VecDeque::new(); 
    neighbor_deque.push_back("you");

    while !neighbor_deque.is_empty() {
        let investigation_subject = neighbor_deque.pop_front().expect("no que");

        if investigation_subject.ends_with('m') {
            println!();
            print!("{investigation_subject}");
            return String::from(" is the mango seller!")
        }

        let Some(neighbors) = get_neighbors(investigation_subject, &neighbors_network) else {
            continue
        };

        add_neighbors_to_que(&mut neighbor_deque, neighbors);
        /*
        for out_neighbor in neighbors {
            neighbor_deque.push_back(neighbor);
        }
        */
    }

    String::from("\nNo mango seller near by...")
}
