impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_by_key(|v| v[1] - v[0]);
        tasks.iter().fold(0, |acc, task| task[1].max(acc + task[0]))
    }
}
