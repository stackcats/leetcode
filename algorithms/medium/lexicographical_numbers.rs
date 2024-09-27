impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut arr = vec![0; n as usize];
        let mut curr = 1;
        for i in 0..n {
            arr[i as usize] = curr;
            if curr * 10 <= n {
                curr *= 10;
                continue;
            }
            while curr >= n || curr % 10 == 9 {
                curr /= 10;
            }
            curr += 1;
        }
        arr
    }
}
