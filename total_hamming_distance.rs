// https://leetcode.com/problems/total-hamming-distance/description/

impl Solution {
    pub fn total_hamming_distance(nums: &mut Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            // // 查看数字二进制格式每一列1和0的个数

            let mut numberOfOnes = 0;
            for n in nums.iter() {
                numberOfOnes += (*n >> i) & 1;
            }

            let numberOfZeroes = nums.len() as i32 - numberOfOnes;
            ans += numberOfOnes * numberOfZeroes;
        }

        ans
    }
}
