fn main() {
    let mut vec: Vec<u32> = vec![14, 81, 97, 1, 8, 92, 23, 82, 46, 16];

    let max = max_with_recursion(&mut vec);
    println!("The max in array is: {max}");
}

fn max_with_recursion(vec: &mut Vec<u32>) -> u32 {
    
    let max: u32 = if vec.is_empty() { 
        0
    } else {
        let popped = vec.pop().unwrap();
        let fn_val = max_with_recursion(vec);
        println!("{popped}");
        if popped > fn_val {
            println!("if brace {popped}");
            popped
        } else {
            println!("else brace {fn_val}", );
            fn_val
            // code that returns when false
        }
    };

    max 
}
