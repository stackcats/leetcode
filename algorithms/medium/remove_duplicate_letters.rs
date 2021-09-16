#[inline]
fn ndx(b: u8) -> usize {
    (b - b'a') as usize
}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts = vec![0; 26];
        for &b in s.as_bytes() {
            counts[ndx(b)] += 1;
        }

        let mut arr = Vec::new();
        let mut set = vec![false; 26];
        for b in s.bytes() {
            counts[ndx(b)] -= 1;

            if set[ndx(b)] {
                continue;
            }

            while !arr.is_empty() && arr[arr.len() - 1] >= b {
                let b = arr[arr.len() - 1];
                if counts[ndx(b)] > 0 {
                    arr.pop();
                    set[ndx(b)] = false;
                } else {
                    break;
                }
            }
            arr.push(b);
            set[ndx(b)] = true;
        }

        arr.into_iter().map(|b| b as char).collect()
    }
}
