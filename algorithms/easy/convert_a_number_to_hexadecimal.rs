impl Solution {
    pub fn to_hex(mut num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut bits = vec![0; 32];
        let is_negative = num < 0;
        num = num.abs();
        let mut i = 31;
        while num > 0 && i >= 0 {
            let r = num % 2;
            bits[i as usize] = r;
            num /= 2;
            i -= 1;
        }
        if is_negative {
            bits[0] = 1;
            for i in 1..32 {
                if bits[i] == 1 {
                    bits[i] = 0;
                } else {
                    bits[i] = 1;
                }
            }
            let mut prime = 1;
            let mut i = 31;
            while prime == 1 && i >= 1 {
                let n = prime + bits[i];
                prime = n / 2;
                bits[i] = n % 2;
                i -= 1;
            }
        }
        let mut ans = String::new();
        let arr = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut is_leading = true;
        for i in (0..32).step_by(4) {
            let n = bits[i] * 8 + bits[i + 1] * 4 + bits[i + 2] * 2 + bits[i + 3];
            if is_leading && n == 0 {
                continue;
            }
            is_leading = false;
            ans.push(arr[n as usize]);
        }
        ans
    }
}
