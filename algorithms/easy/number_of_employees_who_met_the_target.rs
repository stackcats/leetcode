impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours
            .into_iter()
            .fold(0, |acc, h| acc + if h >= target { 1 } else { 0 })
    }
}
