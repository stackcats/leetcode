impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut b5 = 0;
        let mut b10 = 0;
        for b in &bills {
            if *b == 5 {
                b5 += 1;
            } else if *b == 10 {
                if b5 > 0 {
                    b5 -= 1;
                    b10 += 1;
                } else {
                    return false;
                }
            } else {
                if b10 > 0 && b5 > 0 {
                    b10 -= 1;
                    b5 -= 1;
                } else if b5 >= 3 {
                    b5 -= 3;
                } else {
                    return false;
                }
            }
        }

        true
    }
}
