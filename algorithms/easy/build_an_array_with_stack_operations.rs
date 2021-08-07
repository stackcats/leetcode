impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 1;
        let mut arr = Vec::new();
        for t in &target {
            while i < *t {
                arr.push("Push".to_string());
                arr.push("Pop".to_string());
                i += 1;
            }
            arr.push("Push".to_string());
            i += 1;
            if i > n {
                break;
            }
        }
        arr
    }
}
