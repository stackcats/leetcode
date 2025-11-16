impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let mut a = -200;
        let mut b = -200;
        let mut c = 200;
        for n in nums {
            if a < n {
                b = a;
                a = n;
            } else if b < n {
                b = n;
            }
            c = c.min(n);
        }

        a + b - c
    }
}
