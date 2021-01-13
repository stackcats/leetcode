impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![first];
        for e in &encoded {
            let last = ans.last().unwrap();
            ans.push(*e ^ last);
        }
        ans
    }
}
