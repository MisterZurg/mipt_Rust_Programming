#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    get_combination(arr, 0, k, vec![], &mut result);

    return result;
}

fn get_combination(arr: &[i32], l: usize, r: usize, comb: Vec<i32>, result: &mut Vec<Vec<i32>>){
    if r == 0 {
        result.push(comb);
        return;
    }

    for i in l..arr.len() {
        let mut tmp = comb.clone();
        tmp.push(arr[i]);
        get_combination(arr, i + 1, r - 1, tmp, result)
    }
}