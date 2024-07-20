fn height(i: i32, a: i32, b: i32) -> i32 {
    if i <= a {
        height(i + 1, b, a - i)
    } else {
        i - 1
    }
}

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        height(1, red, blue).max(height(1, blue, red))
    }
}
