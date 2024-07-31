impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let n = x.min(y / 4);
        if n % 2 == 1 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}
