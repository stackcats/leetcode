impl Solution {
    pub fn largest_even(s: String) -> String {
        let t = s
            .chars()
            .rev()
            .skip_while(|c| *c == '1')
            .collect::<String>();
        t.chars().rev().collect::<_>()
    }
}
