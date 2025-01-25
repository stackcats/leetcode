fn can_place(force: i32, position: &Vec<i32>, m: i32) -> bool {
    let mut last_position = position[0];
    let mut placed_balls = 1;
    for i in 1..position.len() {
        if position[i] - last_position >= force {
            last_position = position[i];
            placed_balls += 1;
        }
        if placed_balls == m {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let mut low = 1;
        let mut high = position.last().unwrap() - position[0];
        let mut ans = 0;
        while low <= high {
            let mid = low + (high - low) / 2;
            if can_place(mid, &position, m) {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        ans
    }
}
