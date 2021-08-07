impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        for i in 0..arr.len() {
            let mut num_of_repeated = 1;
            for j in ((i + m)..arr.len()).step_by(m) {
                let mut is_equal = true;
                if j + m > arr.len() {
                    break;
                }
                if &arr[(j - m)..j] == &arr[j..(j + m)] {
                    num_of_repeated += 1;
                    if num_of_repeated >= k {
                        return true;
                    }
                } else {
                    num_of_repeated = 1;
                }
            }
        }
        false
    }
}
