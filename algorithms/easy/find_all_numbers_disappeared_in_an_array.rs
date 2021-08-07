impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; nums.len() + 1];
        for n in &nums {
            arr[*n as usize] = 1;
        }
        let mut ans = Vec::new();
        for i in 1..arr.len() {
            if arr[i] == 0 {
                ans.push(i as i32);
            }
        }
        ans
    }
}
