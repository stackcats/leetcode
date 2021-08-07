impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut number_of_left_parentheses = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == '(' {
                number_of_left_parentheses += 1;
                if ans < number_of_left_parentheses {
                    ans = number_of_left_parentheses;
                }
            } else if c == ')' {
                number_of_left_parentheses -= 1;
            }
        }
        ans
    }
}
