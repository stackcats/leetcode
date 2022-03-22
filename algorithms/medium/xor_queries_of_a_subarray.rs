impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = vec![0];
        for (i, &n) in arr.iter().enumerate() {
            let p = prefix[i] ^ n;
            prefix.push(p);
        }
        let mut ans = Vec::new();
        for q in &queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            ans.push(prefix[l] ^ prefix[r + 1]);
        }
        ans
    }
}
