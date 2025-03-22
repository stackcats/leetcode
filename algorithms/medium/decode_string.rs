fn rec(arr: &Vec<char>, mut i: usize) -> (String, usize) {
    let mut s = String::new();
    let mut n = 0;
    while i < arr.len() {
        match arr[i] {
            '0'..='9' => {
                n = n * 10 + (arr[i].to_digit(10).unwrap() as usize);
            }
            '[' => {
                let (t, j) = rec(arr, i + 1);
                s.push_str(&t.repeat(n));
                n = 0;
                i = j;
            }
            ']' => {
                return (s, i);
            }
            _ => {
                s.push(arr[i]);
            }
        }
        i += 1;
    }

    (s, i)
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let arr: Vec<char> = s.chars().collect();
        rec(&arr, 0).0
    }
}
