use std::time::Instant;
mod sorting;
mod two_sum;
mod linked_list;

fn main() {
    let veccy1 = vec![23,11,55,2,32,1,98923,23,11,55];
    let mut start = Instant::now();
    let result_qs = sorting::qs(veccy1);
    let mut duration = start.elapsed();
    println!("Quick sort: {:?}", result_qs);
    println!("Elapsed: {:?}", duration);

    let veccy2 = vec![23,11,55,2,32,1,98923,23,11,55];
    start = Instant::now();
    let result_merge = sorting::merge_sort(veccy2);
    duration = start.elapsed();
    println!("Merge sort: {:?}", result_merge);
    println!("Elapsed: {:?}", duration);
}
