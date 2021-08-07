impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let arr = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i <= j {
            if arr[i] != arr[j] {
                return 2;
            }
            i += 1;
            j -= 1;
        }
        1
    }
}
