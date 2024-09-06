impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut xs = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            if *n == x {
                xs.push(i as i32);
            }
        }

        let mut ans = Vec::new();
        for q in queries {
            let i = (q - 1) as usize;
            ans.push(*xs.get(i).unwrap_or(&-1));
        }
        ans
    }
}
