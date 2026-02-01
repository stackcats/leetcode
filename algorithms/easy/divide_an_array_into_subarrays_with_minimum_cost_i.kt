class Solution {
  fun minimumCost(nums: IntArray): Int = nums[0] + nums.drop(1).sorted().take(2).sum()
}
