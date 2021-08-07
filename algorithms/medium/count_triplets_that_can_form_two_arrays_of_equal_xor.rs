impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..arr.len() {
            let mut x = arr[i];
            for k in (i + 1)..arr.len() {
                x ^= arr[k];
                if x == 0 {
                    ans += k - i;
                }
            }
        }
        ans as i32
    }
}
