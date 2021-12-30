impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let pos = (start_pos[0], start_pos[1]);
        let mut ans = vec![0; s.len()];
        let mut s = s.as_bytes();
        for i in 0..s.len() {
            let mut pos = (start_pos[0], start_pos[1]);
            let mut ct = 0;
            for j in i..s.len() {
                pos = match s[j] {
                    b'R' => (pos.0, pos.1 + 1),
                    b'L' => (pos.0, pos.1 - 1),
                    b'U' => (pos.0 - 1, pos.1),
                    b'D' => (pos.0 + 1, pos.1),
                    _ => unreachable!(),
                };
                if pos.0 >= 0 && pos.0 < n && pos.1 >= 0 && pos.1 < n {
                    ct += 1;
                } else {
                    break;
                }
            }
            ans[i] = ct;
        }
        ans
    }
}
