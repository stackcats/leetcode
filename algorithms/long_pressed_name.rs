impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if name.len() > typed.len() {
            return false;
        }
        let name: Vec<char> = name.chars().collect();
        let typed: Vec<char> = typed.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while i < name.len() && j < typed.len() {
            if name[i] == typed[j] {
                i += 1;
                j += 1;
                continue;
            }
            if name[i] != typed[j] {
                if j > 0 && typed[j] == typed[j - 1] {
                    j += 1;
                    continue;
                }
            }
            return false;
        }
        if i != name.len() {
            return false;
        }
        while j > 0 && j < typed.len() && typed[j] == typed[j - 1] {
            j += 1;
        }
        j == typed.len()
    }
}
