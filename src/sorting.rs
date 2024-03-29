pub fn qs(veccy: Vec<i32>) -> Vec<i32> {
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

pub fn merge_sort(veccy: Vec<i32>) -> Vec<i32> {
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

#[test]
fn test_merge_sort() {
    assert_eq!(merge_sort(vec![4,1,7]), vec![1,4,7]);
}

#[test]
fn test_quick_sort() {
    assert_eq!(qs(vec![4,1,7]), vec![1,4,7]);
}
