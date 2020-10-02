use std::cmp::Ordering;
impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(Solution::sort);
        arr
    }
    fn number_of_ones(mut n: i32) -> i32 {
        let mut num = 0;
        while n > 0 {
            if n % 2 == 1 {
                num += 1;
            }
            n /= 2;
        }
        num
    }
    fn sort<'r, 's>(a: &'r i32, b: &'s i32) -> Ordering {
        let na = Solution::number_of_ones(*a);
        let nb = Solution::number_of_ones(*b);
        if na == nb {
            return a.cmp(b);
        }
        na.cmp(&nb)
    }
}
