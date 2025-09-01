impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for n in order {
            if friends.binary_search(&n).is_ok() {
                ans.push(n);
            }
        }
        ans
    }
}
