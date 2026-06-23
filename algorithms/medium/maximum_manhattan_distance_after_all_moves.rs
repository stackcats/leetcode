impl Solution {
    pub fn max_distance(moves: String) -> i32 {
        let moves = moves.as_bytes();
        let (mut x, mut y) = (0i32, 0i32);
        let mut ct = 0;
        for m in moves {
            match m {
                b'U' => x -= 1,
                b'D' => x += 1,
                b'L' => y -= 1,
                b'R' => y += 1,
                _ => ct += 1,
            }
        }

        x.abs() + y.abs() + ct
    }
}
