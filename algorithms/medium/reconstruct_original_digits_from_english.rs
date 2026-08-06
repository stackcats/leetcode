impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut ct = [0; 10];
        for c in s.chars() {
            match c {
                'z' => ct[0] += 1,
                'w' => ct[2] += 1,
                'x' => ct[6] += 1,
                'u' => ct[4] += 1,
                'f' => ct[5] += 1,
                'o' => ct[1] += 1,
                's' => ct[7] += 1,
                'r' => ct[3] += 1,
                'g' => ct[8] += 1,
                'i' => ct[9] += 1,
                _ => {}
            }
        }

        ct[5] -= ct[4]; // f
        ct[1] -= ct[0] + ct[2] + ct[4]; // o
        ct[7] -= ct[6]; // s
        ct[3] -= ct[0] + ct[4]; // r
        ct[9] -= ct[6] + ct[8] + ct[5]; // i

        let mut s = String::new();
        for i in 0..10 {
            while ct[i] > 0 {
                s.push((b'0' + i as u8) as char);
                ct[i] -= 1;
            }
        }
        s
    }
}

/*
zero
one
two
three
four
five
six
seven
eight
nine
*/
