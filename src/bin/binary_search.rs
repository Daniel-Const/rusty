fn binary_search(vec: Vec<i32>, min: usize, max: usize, needle: i32) -> i32 {
    if min > max {
        return -1;
    }
    let partition: usize = (min + max) / 2;
    let value: i32 = vec[partition];
    return match value.cmp(&needle) {
        std::cmp::Ordering::Less    => binary_search(vec, partition+1, max, needle),
        std::cmp::Ordering::Greater => binary_search(vec, min, partition-1, needle),
        std::cmp::Ordering::Equal   => partition.try_into().unwrap()
    }
}

fn perform_search(vec: Vec<i32>, needle: i32) {
    let result = binary_search(vec.clone(), 0, vec.len()-1, needle);
    println!("Is {} in {:?} ?", needle, vec);
    match result.cmp(&0) {
        std::cmp::Ordering::Greater => println!("Yes at position {}", result),
        _ => println!("No")
    }

}

fn main() {
    let arr = vec![2, 4, 6, 8, 10, 11, 14, 15];
    perform_search(arr.clone(), 12);
    perform_search(arr.clone(), 14);

    perform_search(rusty::generate_vec(20, 0, 100), 54);
}

