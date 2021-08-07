impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut frogs: [i32; 5] = [0; 5];
        let mut ans = 0;
        for c in croak_of_frogs.chars() {
            let i = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => 5,
            };
            if i == 0 {
                frogs[0] += 1;
                if frogs[4] > 0 {
                    frogs[4] -= 1;
                } else {
                    ans += 1;
                }
            } else {
                frogs[i] += 1;
                frogs[i - 1] -= 1;
                if frogs[i - 1] < 0 {
                    return -1;
                }
            }
        }
        for i in 0..4 {
            if frogs[i] != 0 {
                return -1;
            }
        }
        ans
    }
}
