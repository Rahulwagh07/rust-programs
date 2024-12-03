use priority_queue::PriorityQueue; 
use std::cmp::Reverse;

pub fn main() {
    let arr: Vec<Vec<i32>> = vec![
        vec![16, 28, 60, 64],
        vec![22, 41, 63, 91],
        vec![27, 50, 87, 93],
        vec![36, 78, 87, 94],
    ];
    let k = 5;
    let res = kth_smallest(&arr, k);
    
    if res != -1 {
        println!("The {}th smallest element is: {}", k, res);
    } else {
        println!("There are fewer than {} elements.", k);
    }
}

fn kth_smallest(arr: &Vec<Vec<i32>>, k: i32) -> i32 {
    let mut pq = PriorityQueue::new();
    
    for row in arr {
        for &num in row {
            pq.push(num, Reverse(num));
        }
    }

    let mut count = 0;
    while let Some((num, _)) = pq.pop() {
        count += 1;
        if count == k {
            return num; 
        }
    }
    
    -1  
}