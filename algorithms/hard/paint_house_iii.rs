fn calc_cost(dp: &mut Vec<Vec<Vec<i32>>>, n: usize, cost: i32, h: usize, g: usize, c: usize) {
    for pre_color in 1..=n {
        let pre_cost = if pre_color == c {
            dp[h - 1][g][pre_color]
        } else {
            dp[h - 1][g - 1][pre_color]
        };

        dp[h][g][c] = dp[h][g][c].min(pre_cost + cost);
    }
}

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let max = 10000000;
        let mut dp = vec![vec![vec![max; n + 1]; m + 1]; m + 1];
        for c in 1..=n {
            dp[0][0][c] = 0;
        }

        for h in 1..=m {
            for g in 1..=h {
                if houses[h - 1] == 0 {
                    for color in 1..=n {
                        calc_cost(&mut dp, n, cost[h - 1][color - 1], h, g, color);
                    }
                } else {
                    calc_cost(&mut dp, n, 0, h, g, houses[h - 1] as usize);
                }
            }
        }

        let mut ans = max;
        for c in 1..=n {
            ans = ans.min(dp[m][target as usize][c]);
        }

        if ans == max {
            -1
        } else {
            ans
        }
    }
}
