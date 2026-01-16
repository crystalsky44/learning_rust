use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7 ,8, 9, 10];

    let mut input = String::new();
    print!("input a number ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end().parse::<u32>().unwrap();
    

    match recursive_binary(&mut vec, input) {
        Ok(index) => {
            println!("The return of recursive_binary: {}", vec[index]);
        }
        Err(message) => {
            println!("{message}");
        } 
    }
}

fn recursive_binary(vec: &mut Vec<u32>, key: u32) -> Result<usize, &str> {
    let mut middle: usize;

    if vec.len() == 1 {
        if vec[0] == key {
            return Ok(0)
        } 
        return Err("no match found")
    } 

    let high: usize = vec.len() - 1;
    let low: usize = 0;
    middle = high / 2;

    /*
    if vec[middle] > key {
        vec.drain(middle..=high);
    } else if vec[middle] < key {
        vec.drain(low..=middle);
    } else {
        return Ok(middle);
    } 
    */

    match vec[middle].cmp(&key) {
        Ordering::Greater => vec.drain(middle..=high),
        Ordering::Less => vec.drain(low..=middle),
        Ordering::Equal => return Ok(middle),
    };

    middle = recursive_binary(vec, key)?;

    Ok(middle)
    // vec[middle] this should prove if the key and the value stored at
    // position euqals
}

