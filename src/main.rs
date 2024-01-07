use std::time::{Duration, Instant};

fn qs(veccy: Vec<i32>) -> Vec<i32> {
    if veccy.len() <= 1 {
        return veccy;
    }
    let pivot = veccy[0];
    let lt = veccy.clone().into_iter().filter(|n| n < &pivot).collect::<Vec<i32>>();
    let gt = veccy.into_iter().filter(|n| n > &pivot).collect::<Vec<i32>>();
    
    return [qs(lt), vec![pivot], qs(gt)].concat();
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut left_i = 0;
    let mut right_i = 0;
    while left_i < left.len() && right_i < right.len() {
        if left[left_i] < right[right_i] {
            result.push(left[left_i]);
            left_i += 1;
        } else {
            result.push(right[right_i]);
            right_i += 1;
        }
    }

    while left_i < left.len() {
        result.push(left[left_i]);
        left_i += 1;
    }
    while right_i < right.len() {
        result.push(right[right_i]);
        right_i += 1;
    }
    return result;
}

fn merge_sort(veccy: Vec<i32>) -> Vec<i32> {
    if veccy.len() <= 1 {
        return veccy;
    }
    
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 0..veccy.len() {
        if i < veccy.len() / 2 {
            left.push(veccy[i]);
        } else {
            right.push(veccy[i]);
        }
    }

    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);


    let merged = merge(sorted_left, sorted_right);
    return merged;
}


fn main() {
    let veccy1 = vec![23,11,55,2,32,1,98923,23,11,55];
    let mut start = Instant::now();
    let result_qs = qs(veccy1);
    let mut duration = start.elapsed();
    println!("Quick sort: {:?}", result_qs);
    println!("Elapsed: {:?}", duration);

    let veccy2 = vec![23,11,55,2,32,1,98923,23,11,55];
    start = Instant::now();
    let result_merge = merge_sort(veccy2);
    duration = start.elapsed();
    println!("Merge sort: {:?}", result_merge);
    println!("Elapsed: {:?}", duration);
}
