impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut ans = -1;

        for (i, &c) in capacity.iter().enumerate() {
            if c >= item_size && (ans == -1 || capacity[ans as usize] > c) {
                ans = i as i32;
            }
        }

        ans
    }
}
