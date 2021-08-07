impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let s = num.to_string();
        s.replacen('6', "9", 1).parse().unwrap()
    }
}
