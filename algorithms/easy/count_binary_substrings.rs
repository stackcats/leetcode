impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut ans = 0;
        let arr: Vec<char> = s.chars().collect();
        for i in 0..(arr.len() - 1) {
            if arr[i] != arr[i + 1] {
                ans += 1;
                let mut l = i as i32 - 1;
                let mut r = i + 1 + 1;
                while l >= 0 && r < arr.len() && arr[l as usize] == arr[i] && arr[i + 1] == arr[r] {
                    ans += 1;
                    l -= 1;
                    r += 1;
                }
            }
        }
        ans
    }
}
