// Find if there is a subarray (of size at least one) with 0 sum.

use std::collections::HashMap;

fn main() {
    let arr = [4, 2, -3, 1, 6];
    let res = find_sum(&arr);
    println!("{}", res);
}

fn find_sum(arr: &[i32]) -> bool {
    let mut map = HashMap::new();
    let mut sum = 0;
    map.insert(0, true);

    for &value in arr {
        sum += value;
        if map.contains_key(&sum) {
            return true;
        }
        map.insert(sum, true);
    }

    false
}
