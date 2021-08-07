impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        for p in &position {
            if *p % 2 == 0 {
                first += 1;
            } else {
                second += 1;
            }
        }
        first.min(second)
    }
}
