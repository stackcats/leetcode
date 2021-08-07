impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();
        let f = arr.len() * 5 / 100;
        let mut sum = 0;
        for i in f..(arr.len() - f) {
            sum += arr[i];
        }
        (sum as f64) / ((arr.len() - f - f) as f64)
    }
}
