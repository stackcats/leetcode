impl Solution {
    pub fn get_smallest_string(mut n: i32, mut k: i32) -> String {
        let mut s = String::new();        
        while n > 0 {
            let c = 26.min(k - (n - 1));
            s.push((b'a' + c as u8 - 1) as char);
            n -= 1;
            k -= c;
        }
        s.chars().rev().collect()
    }
}
