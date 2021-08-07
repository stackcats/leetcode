impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        for n in left..=right {
            let mut m = n;
            let mut flag = true;
            while m > 0 {
                let d = m % 10;
                if d == 0 || n % d != 0 {
                    flag = false;
                    break;
                }
                m /= 10;
            }
            if flag {
                vec.push(n);
            }
        }
        vec
    }
}
