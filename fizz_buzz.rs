impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        for i in 1..n + 1 {
            if i % 15 == 0 {
                ans.push(String::from("FizzBuzz"))
            } else if i % 3 == 0 {
                ans.push(String::from("Fizz"))
            } else if i % 5 == 0 {
                ans.push(String::from("Buzz"))
            } else {
                ans.push(format!("{}", i));
            }
        }
        ans
    }
}
