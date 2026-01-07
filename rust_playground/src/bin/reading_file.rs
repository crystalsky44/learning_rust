use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("foo.txt").unwrap();
    let mut vec_of_utf = Vec::new();

    f.read_to_end(&mut vec_of_utf).unwrap();

    println!("{vec_of_utf:?}");

    let n_string = String::from_utf8(vec_of_utf).unwrap();


    // println!("The bytes: {:?}", &buffer[..n]);
    println!("This is in string {n_string}");
}


