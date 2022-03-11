impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        let num = num - 3;
        if num % 3 != 0 {
            return vec![];
        }
        let num = num / 3;
        vec![num, num + 1, num + 2]
    }
}
