impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut arr = vec![0; n as usize];
        arr[0] = 1;
        let mut curr = 0;
        let mut i = 1;
        loop {
            curr = (curr + i * (k as usize)) % (n as usize);
            if arr[curr] == 1 {
                break;
            }
            arr[curr] = 1;
            i += 1;
        }
        let mut ans = Vec::new();
        for i in 0..arr.len() {
            if arr[i] == 0 {
                ans.push((i + 1) as i32);
            }
        }
        ans
    }
}
