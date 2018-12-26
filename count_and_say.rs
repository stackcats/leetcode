// https://leetcode.com/problems/count-and-say/

fn read_string(s: String) -> String {
    let mut pre = None;
    let mut ct = 0;
    let mut ans = String::new();

    for c in s.chars() {
        if pre.is_none() {
            pre = Some(c);
            ct = 1;
        } else if pre.unwrap() == c {
            ct += 1;
        } else {
            ans = format!("{}{}{}", ans, ct, pre.unwrap());
            ct = 1;
            pre = Some(c);
        }
    }

    ans = format!("{}{}{}", ans, ct, pre.unwrap());
    ans
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ans = "1".to_string();
        for _ in 1..n {
            ans = read_string(ans);
        }

        ans
    }
}
