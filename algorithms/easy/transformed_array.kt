class Solution {
  fun ndx(i: Int, offset: Int, size: Int): Int {
    return ((i + offset) % size + size) % size
  }

  fun constructTransformedArray(nums: IntArray): IntArray {
    val ans = IntArray(nums.size)
    for (i in nums.indices) {
      val j = ndx(i, nums[i], nums.size)
      ans[i] = nums[j]
    }
    return ans
  }
}
