impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut highest = -1;
        let mut second_highest = -1;
        for c in s.chars() {
            if let Some(n) = c.to_digit(10) {
                let n = n as i32;
                if highest < n {
                    second_highest = highest;
                    highest = n;
                } else if second_highest < n && n < highest {
                    second_highest = n;
                }
            }
        }
        second_highest
    }
}
