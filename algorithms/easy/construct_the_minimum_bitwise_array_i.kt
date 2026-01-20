class Solution {
  fun minBitwiseArray(nums: List<Int>): IntArray {
    val ans = IntArray(nums.size)
    for ((i, n) in nums.withIndex()) {
      if (n % 2 == 0) {
        ans[i] = -1
        continue
      }

      var m = 1
      while (m or (m + 1) != n) {
        m += 1
      }
      ans[i] = m
    }

    return ans
  }
}
