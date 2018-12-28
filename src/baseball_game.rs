// https://leetcode.com/problems/baseball-game/

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut score = Vec::new();
        for s in ops.iter() {
            if s == "C" {
                score.pop();
            } else if s == "D" {
                let last = score[score.len() - 1];
                score.push(last * 2);
            } else if s == "+" {
                let last = score[score.len() - 1];
                let last2nd = score[score.len() - 2];
                score.push(last + last2nd);
            } else {
                score.push(s.parse::<i32>().unwrap());
            }
        }

        score.iter().fold(0, |acc, x| acc + x)
    }
}
