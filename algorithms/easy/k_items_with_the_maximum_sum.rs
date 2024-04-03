impl Solution {
    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, mut k: i32) -> i32 {
        let mut sum = num_ones.min(k);
        k -= sum;
        k -= k.min(num_zeros);
        sum -= k.min(num_neg_ones);
        sum
    }
}
