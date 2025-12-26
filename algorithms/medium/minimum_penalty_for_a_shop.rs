impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut ys = customers.chars().filter(|c| *c == 'Y').count();
        let mut ns = 0;
        let mut ans = 0;
        let mut min = ys;
        for (i, c) in customers.chars().enumerate() {
            if min > ys + ns {
                min = ys + ns;
                ans = i;
            }

            if c == 'Y' {
                ys -= 1;
            } else {
                ns += 1;
            }
        }

        if min > ns {
            ans = customers.len();
        }

        ans as _
    }
}
