impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut target = arr.len() as i32;
        let mut ans = Vec::new();
        while target > 1 {
            let mut i = 0;
            while i < arr.len() {
                if arr[i] == target {
                    ans.push(i as i32 + 1);
                    arr[..=i].reverse();
                    ans.push(target);
                    arr[..(target as usize)].reverse();
                    target -= 1;
                    break;
                }
                i += 1;
            }
        }
        ans
    }
}
