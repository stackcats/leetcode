fn can_divided(str1: &str, str2: &str) -> bool {
    let a1 = str1.chars().collect::<Vec<char>>();
    let a2 = str2.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut j = 0;
    while i < a1.len() {
        if j == a2.len() {
            j = 0;
        }
        if a1[i] != a2[j] {
            return false;
        }
        i += 1;
        j += 1;
    }
    i == a1.len() && j == a2.len()
}
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut len = str1.len();
        let mut s = &str1;
        if len > str2.len() {
            len = str2.len();
            s = &str2;
        }
        while len > 0 {
            if can_divided(&str1, &s[0..len]) && can_divided(&str2, &s[0..len]) {
                return s[0..len].to_string();
            }
            len -= 1;
        }
        "".to_string()
    }
}
