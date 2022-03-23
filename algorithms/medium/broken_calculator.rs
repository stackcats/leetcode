impl Solution {
    pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
        let mut n = 0;
        while start_value < target {
            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
            n += 1;
        }

        n + start_value - target
    }
}
