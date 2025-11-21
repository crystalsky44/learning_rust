fn main() {
    let my_list = [1, 3, 5, 7, 9];

    let position = binary_search(&my_list, 3);
    println!("position of the item is: {position}");
    println!("the value in position {position} is {}", my_list[position]);
}

fn binary_search(list: &[i32], item_in_serach: i32) -> usize {
    let mut low = 0;
    let mut high = list.len() - 1;
    let mut mid: usize;
    let mut value_in_mid: i32;
    
    while low <= high {
        mid = usize::midpoint(low, high);
        value_in_mid = list[mid];
        if item_in_serach == value_in_mid { return mid }

        if item_in_serach > value_in_mid { 
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    panic!();
}
