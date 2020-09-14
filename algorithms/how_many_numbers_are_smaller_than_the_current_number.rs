impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut map = vec![0; 101];
        for n in &nums {
            map[*n as usize] += 1;
        }
        for i in 1..101 {
            map[i] += map[i-1];
        }
        let mut arr = Vec::new();
        for n in &nums {
            if *n == 0 {
                arr.push(*n);
            } else {
                arr.push(map[(*n - 1) as usize]);
            }
        }
        arr
    }
}
