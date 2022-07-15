impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() > 12 {
            return vec![];
        }

        let s = s.as_bytes();
        let mut ans = Vec::new();
        for i in 1..s.len() {
            if i > 1 && s[0] == b'0' {
                break;
            }
            let a = String::from_utf8_lossy(&s[..i]).parse::<i32>().unwrap();
            if a > 255 {
                break;
            }
            for j in i + 1..s.len() {
                if j - i > 1 && s[i] == b'0' {
                    break;
                }
                let b = String::from_utf8_lossy(&s[i..j]).parse::<i32>().unwrap();
                if b > 255 {
                    break;
                }
                for k in j + 1..s.len() {
                    if k - j > 1 && s[j] == b'0' {
                        break;
                    }

                    let c = String::from_utf8_lossy(&s[j..k]).parse::<i32>().unwrap();
                    if c > 255 {
                        break;
                    }

                    if s.len() - k > 1 && s[k] == b'0' {
                        continue;
                    }
                    let d = String::from_utf8_lossy(&s[k..]).parse::<i32>().unwrap();
                    if d <= 255 {
                        ans.push(format!("{}.{}.{}.{}", a, b, c, d));
                    }
                }
            }
        }

        ans
    }
}
