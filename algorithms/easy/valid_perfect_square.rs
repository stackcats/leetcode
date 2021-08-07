impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let d = num % 10;
        if d != 0 && d != 1 && d != 4 && d != 9 && d != 6 && d != 5 {
            return false;
        }
        for i in 1..=num {
            if i * i == num {
                return true;
            }
            if i * i > num {
                break;
            }
        }
        false
    }
}
