impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1: Vec<i32> = version1
            .split(".")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut version2: Vec<i32> = version2
            .split(".")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        while !version1.is_empty() && version1[version1.len() - 1] == 0 {
            version1.pop();
        }
        while !version2.is_empty() && version2[version2.len() - 1] == 0 {
            version2.pop();
        }
        let mut i = 0;
        let mut j = 0;
        while i < version1.len() && j < version2.len() {
            if version1[i] < version2[j] {
                return -1;
            }
            if version1[i] > version2[j] {
                return 1;
            }
            i += 1;
            j += 1;
        }
        if i < version1.len() {
            return 1;
        }
        if j < version2.len() {
            return -1;
        }

        0
    }
}
