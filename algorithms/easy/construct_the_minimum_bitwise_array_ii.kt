class Solution {
  fun minBitwiseArray(nums: List<Int>): IntArray {
    val ans = IntArray(nums.size) { -1 }

    for ((i, n) in nums.withIndex()) {
      var d = 1
      while (n and d != 0) {
        ans[i] = n - d
        d = d shl 1
      }
    }

    return ans
  }
}
