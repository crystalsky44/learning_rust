fn main() {
    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7 ,8, 9, 10];

    let count = count_with_recursion(&mut vec);
    println!("The elements in array is: {count}");
}

fn count_with_recursion(vec: &mut Vec<u32>) -> u32 {
    
    let count: u32 = if vec.is_empty() {
        0
    } else {
        vec.pop().unwrap();
        1 + count_with_recursion(vec)
    };

    count 
}
