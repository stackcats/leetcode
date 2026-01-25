class Solution {
  fun minimumDifference(nums: IntArray, k: Int): Int {
    nums.sort()

    var ans = Int.MAX_VALUE
    for (i in (k - 1)..<nums.size) {
      ans = min(ans, nums[i] - nums[i - k + 1])
    }

    return ans
  }
}
