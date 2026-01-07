#![warn(clippy::pedantic)]

use std::collections::{HashMap, VecDeque};

fn get_neighbors<'a>(key: &str, neighbors_network: HashMap<&str, Option<Vec<&'a str>>>) ->
Option<Vec<&'a str>> {
    match neighbors_network.get(key).expect("check your key") {
        Some(neighbors) => {
            println!("{neighbors_network:?}");
            Some(neighbors.clone())
        },
        None => {
            println!("No one found!");
            None
        }
    }
}

fn add_neighbors_to_que<'a>(neighbor_deque: &mut VecDeque<&'a str>, neighbors: Vec<&'a str>) {
    for neighbor in neighbors {
        neighbor_deque.push_back(neighbor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_que() {
        let mut deque = VecDeque::from(["alice"]);
        let vec = Vec::from(["bob", "claire"]);

        add_neighbors_to_que(&mut deque, vec);

        let result = VecDeque::from(["alice", "bob", "claire"]);
        println!("{deque:?}");
        assert_eq!(deque, result);
    }

    #[test]
    fn test_one() {
        let neighbors_network = HashMap::from([
            ("you", Some(vec!["alice", "bob", "claire"])),
            ("claire", None)
        ]);

        let key = "you";
        let test: Option<Vec<&str>> = get_neighbors(key, neighbors_network);

        assert_eq!(test, vec!["alice", "bob", "claire"].into());
    }

    #[test]
    fn test_two() {
        let neighbors_network = HashMap::from([
            ("you", Some(vec!["alice", "bob", "claire"])),
            ("claire", None)
        ]);

        let key = "claire";
        let test: Option<Vec<&str>> = get_neighbors(key, neighbors_network);

        assert_eq!(test, None);
    }
}
