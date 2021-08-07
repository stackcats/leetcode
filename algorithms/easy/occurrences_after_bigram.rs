impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let arr = text.split(" ").collect::<Vec<&str>>();
        let mut ans = Vec::new();
        for i in 0..(arr.len() - 1) {
            if arr[i] == first && arr[i+1] == second && i + 2 < arr.len() {
                ans.push(arr[i+2].to_string());
            }
        }
        ans
    }
}
