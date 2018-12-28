// https://leetcode.com/problems/dungeon-game/description/

impl Solution {
    pub fn calculate_minimum_hp(dungeon: &mut Vec<Vec<i32>>) -> i32 {
        let row = dungeon.len();
        let col = dungeon[0].len();
        let mut d = vec![vec![i32::max_value(); col + 1]; row + 1];
        d[row][col - 1] = 1;
        d[row - 1][col] = 1;
        for i in (0..row).rev() {
            for j in (0..col).rev() {
                d[i][j] = 1.max(d[i + 1][j].min(d[i][j + 1]) - dungeon[i][j]);
            }
        }

        d[0][0]
    }
}
