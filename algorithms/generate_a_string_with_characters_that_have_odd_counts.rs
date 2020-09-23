impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n == 1 {
            return "a".to_string();
        }
        let mut arr = Vec::new();
        if n % 2 == 0 {
            arr.push('a');
            for i in 1..n {
                arr.push('b');
            }
        } else {
            arr.push('a');
            arr.push('b');
            for i in 2..n {
                arr.push('c');
            }
        }
        arr.iter().collect::<String>()
    }
}
