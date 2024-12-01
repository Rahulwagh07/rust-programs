use std::collections::HashMap;

fn main() {
    let s = vec!["aaa", "bbb", "ccc", "bbb", "aaa", "aaa", "ccc", "ccc", "ccc"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let res = sec_frequent(s);
    println!("{}", res);
}

fn sec_frequent(arr: Vec<String>) -> String {
    let mut count_map = HashMap::new();

    for s in arr {  //here ownership of each ele in arr is paseed to the loop
        *count_map.entry(s).or_insert(0) += 1;
    }

    let mut first_max_count = 0;
    let mut second_max_count = 0;

    for &count in count_map.values() {
        if count > first_max_count {
            second_max_count = first_max_count;
            first_max_count = count;
        } else if count > second_max_count && count < first_max_count {
            second_max_count = count;
        }
    }

    for (key, &value) in count_map.iter() {
        if value == second_max_count {
            return key.clone();
        }
    }

    "".to_string()
}
