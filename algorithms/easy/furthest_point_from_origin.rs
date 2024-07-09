impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut ls = 0i32;
        let mut rs = 0i32;
        let mut us = 0i32;
        for c in moves.chars() {
            match c {
                'L' => ls += 1,
                'R' => rs += 1,
                _ => us += 1,
            };
        }
        us + ls.max(rs) - ls.min(rs)
    }
}
