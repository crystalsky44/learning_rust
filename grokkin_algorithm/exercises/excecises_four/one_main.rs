fn main() {
    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7 ,8, 9, 10];

    let sum = sum_with_recursion(&mut vec);
    println!("The sum of array is: {sum}");
}

fn sum_with_recursion(vec: &mut Vec<u32>) -> u32 {
    
    let sum: u32 = if vec.is_empty() {
        0
    } else {
        vec.pop().unwrap() + sum_with_recursion(vec)
    };

    sum
}

