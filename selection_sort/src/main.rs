fn main() {
    let mut array_0 = vec![63, 28, 9, 26, 98, 56, 73, 5, 76, 94];
    let mut array_1: Vec<i32> = Vec::new();

    selection_sort(&mut array_0, &mut array_1);

    for index in array_1 {
        print!("{index} ");
    }

}

fn selection_sort(array_0: &mut Vec<i32>, array_1: &mut Vec<i32>) {
    for _ in 0..array_0.len() {
        let smallest_index = index_of_smallest(array_0);
        array_1.push(array_0.swap_remove(smallest_index));
    }
}

fn index_of_smallest(array_0: &mut [i32]) -> usize {
    let mut index_of_smallest: usize = 0;
    let mut smallest_value: i32 = array_0[index_of_smallest];

    for (index, item) in array_0.iter().enumerate().skip(1) {
        if *item < smallest_value {
            smallest_value = *item;
            index_of_smallest = index;
        }
    }

    index_of_smallest
}
