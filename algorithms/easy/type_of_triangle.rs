fn is_valid(a: i32, b: i32, c: i32) -> bool {
    a + b > c && a + c > b && b + c > a
}
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        let s = if !is_valid(a, b, c) {
            "none"
        } else if a == b && b == c {
            "equilateral"
        } else if a == b || b == c || a == c {
            "isosceles"
        } else {
            "scalene"
        };
        s.to_string()
    }
}
