impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        10 * (additional_tank.min((main_tank - 1) / 4) + main_tank)
    }
}
