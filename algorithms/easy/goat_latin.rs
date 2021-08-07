impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut ans = Vec::new();
        let mut i = 0;
        for word in s.split(" ") {
            ans.push(Solution::trans(&word, i));
            i += 1;
        }
        ans.join(" ")
    }
    fn trans(from: &str, n: i32) -> String {
        let mut to = match from.chars().nth(0).unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => from.to_string() + &"ma",
            _ => from[1..].to_string() + &from[0..1] + &"ma",
        };
        for _ in 0..=n {
            to.push('a');
        }
        to
    }
}
