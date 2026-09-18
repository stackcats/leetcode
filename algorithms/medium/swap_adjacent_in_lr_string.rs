impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        if start.len() != result.len() {
            return false;
        }

        let start = start.as_bytes();
        let result = result.as_bytes();

        let mut i = 0;
        let mut j = 0;

        loop {
            while i < start.len() && start[i] == b'X' {
                i += 1;
            }

            while j < result.len() && result[j] == b'X' {
                j += 1;
            }

            if i == start.len() && j == result.len() {
                return true;
            }

            if i == start.len() || j == result.len() {
                return false;
            }

            if start[i] != result[j] {
                return false;
            }

            if start[i] == b'R' {
                if i > j {
                    return false;
                }
            } else {
                if i < j {
                    return false;
                }
            }

            i += 1;
            j += 1;
        }
    }
}
