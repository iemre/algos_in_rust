use std::collections::HashMap;


// return the indices of 2 numbers in a given list of numbers that sum
// up to given target sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut existing_nums = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        existing_nums.insert(num, idx);
    }
    for (idx, num) in nums.iter().enumerate() {
        let remain = target - num;
        match existing_nums.get(&remain) {
            Some(remain_idx) => {
                if *remain_idx == idx {
                    continue;
                } 
                return vec![idx, *remain_idx];
            },
            None => {
                continue;
            }

        }
    }
    return vec![];
}

#[test]
fn test_two_sum() { 
    assert_eq!(two_sum(vec![1,2,3,4], 5), vec![0, 3]);
}

#[test]
fn test_two_sum_not_found() {
    assert_eq!(two_sum(vec![1,5], 2), vec![]);
}

#[test]
fn test_two_sum_empty_input() {
    assert_eq!(two_sum(vec![], 5), vec![]);
}