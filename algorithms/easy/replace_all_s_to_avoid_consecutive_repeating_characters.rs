impl Solution {
    pub fn modify_string(s: String) -> String {
        if s == "?" {
            return "a".to_string();
        }
        let mut chars: Vec<char> = (b'a'..=b'z').map(char::from).collect();
        let mut arr: Vec<char> = s.chars().collect();
        for i in 0..arr.len() {
            if arr[i] == '?' {
                for c in &chars {
                    if i == 0 {
                        if *c != arr[i + 1] {
                            arr[i] = *c;
                            break;
                        }
                    } else if i == arr.len() - 1 {
                        if *c != arr[i - 1] {
                            arr[i] = *c;
                            break;
                        }
                    } else {
                        if *c != arr[i - 1] && *c != arr[i + 1] {
                            arr[i] = *c;
                            break;
                        }
                    }
                }
            }
        }
        arr.iter().collect()
    }
}
