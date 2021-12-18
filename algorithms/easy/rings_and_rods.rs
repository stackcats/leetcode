impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = vec![0; 10];
        let rings = rings.as_bytes();
        for i in (0..rings.len()).step_by(2) {
            let color = match rings[i] {
                b'B' => 1,
                b'R' => 2,
                b'G' => 4,
                _ => 0,
            };
            let ndx = (rings[i + 1] - b'0') as usize;
            rods[ndx] |= color;
        }

        rods.iter()
            .fold(0, |acc, &r| acc + if r == 7 { 1 } else { 0 })
    }
}
