use std::cmp::Ordering;

fn main() {
    // input: sorted, or not sorted list
    // output: sorted list

    // let mut vec: Vec<u32> = vec![87];
    // let mut vec: Vec<u32> = vec![87, 37];
    let mut vec: Vec<u32> = vec![87, 37, 92, 76, 57, 58, 4, 99, 65, 13];

    vec = quick_sort(vec);
    println!("After sort: {vec:?}");
}

fn quick_sort(mut vec: Vec<u32>) -> Vec<u32> {
    // when there are only two elements in the list,
    // compare the first with the second, and if it was
    // greater, swap. if smaller or equal to, return vec
    //
    // one thing I was considering here was, should I use a match of if blocks?

    match vec.len().cmp(&2) {
        Ordering::Less => {
            println!("You're in the Less!");
            return vec
        },

        Ordering::Equal => {
            println!("You're in the Equal!");

            if vec[0] > vec[1] {
                vec.swap(0 ,1);
            }
            return vec
        },

        Ordering::Greater => {
            println!("You're in the Greater!");
        
            // choose a pivot (this program's case, vec[0])
            // creating a maximum of two sub arrays
            // vec_left = element's value smaller than pivot
            // vec_right = element's value greater than pivot

            // create, find, or something to concatinate
            // vec_left + pivot(vec[0]) + vec_right

            // since pivot is a single element, why don't I just push(pivot) it?
            
            // there's a chance one of the Vector might not be used
            // should I implement the creating of vec with dependence?

            // let pivot = vec[0]; *this line won't work, because it doesn't
            // reduce the problem
            let pivot = vec.pop().unwrap();

            let mut vec_less: Vec<u32> = Vec::new();
            let mut vec_greater: Vec<u32> = Vec::new();

            for element in &vec {
                println!("in the for loop: {element}");
                if *element > pivot {
                    vec_greater.push(*element);
                } else {
                    vec_less.push(*element);
                }
            }

            let mut vec_less = quick_sort(vec_less);
            println!("in between two sorts");
            let vec_greater = quick_sort(vec_greater);
            vec_less.push(pivot);
            vec_less.extend(vec_greater);

            vec = vec_less;

        },
    }
    vec
}
