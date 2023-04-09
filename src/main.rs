fn qs(veccy: Vec<i32>) -> Vec<i32> {
    if veccy.len() <= 1 {
        return veccy;
    }
    let pivot = veccy[0];
    let lt = veccy.clone().into_iter().filter(|n| n < &pivot).collect::<Vec<i32>>();
    let gt = veccy.into_iter().filter(|n| n > &pivot).collect::<Vec<i32>>();
    
    return [qs(lt), vec![pivot], qs(gt)].concat();
}

fn main() {
    let veccy = vec![23,11,55,2,32,1,98923];
    let result = qs(veccy);
    for r in result {
        println!("{}", r);
    }
}