impl Solution {
    pub fn max_power(s: String) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut ans = 0;
        while i < arr.len() {
            let mut j = i + 1;
            while j < arr.len() && arr[j] == arr[i] {
                j += 1;
            }
            if ans < j - i {
                ans = j - i;
            }
            i = j;
        }
        ans as i32
    }
}
