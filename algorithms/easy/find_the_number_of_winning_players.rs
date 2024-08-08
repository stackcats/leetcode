impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ct = vec![[0; 11]; n + 1];
        for p in pick {
            ct[p[0] as usize][p[1] as usize] += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            if ct[i].iter().any(|&x| x > i) {
                ans += 1;
            }
        }
        ans
    }
}
