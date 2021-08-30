impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut arr: Vec<&str> = preorder.split(",").collect();
        if arr[0] == "#" {
            return arr.len() == 1;
        }
        let mut ct = 1;
        let mut i = 1;
        while ct > 0 {
            ct -= 1;
            if i >= arr.len() {
                return false;
            }
            if arr[i] != "#" {
                ct += 1;
            }
            i += 1;
            if i >= arr.len() {
                return false;
            }
            if arr[i] != "#" {
                ct += 1;
            }
            i += 1;
        }

        i == arr.len()
    }
}
