fn bi(mut n: i32) -> Vec<char> {
    let mut s = Vec::new();
    while n > 0 {
        if n % 2 == 0 {
            s.push('0');
        } else {
            s.push('1');
        }
        n /= 2;
    }
    s.into_iter().rev().collect()
}
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut ans = 0;
        let s = bi(n);
        let mut flag = false;
        let mut start = 0;
        for i in 0..s.len() {
            if s[i] == '0' {
                continue;
            }
            if flag == false {
                // find the first '1'
                flag = true;
                start = i;
            } else {
                if ans < i - start {
                    ans = i - start;
                }
                start = i;
            }
        }
        ans as i32
    }
}
