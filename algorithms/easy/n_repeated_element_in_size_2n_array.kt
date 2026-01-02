class Solution {
  fun repeatedNTimes(nums: IntArray): Int {
    val st = HashSet<Int>()
    for (n in nums) {
      if (st.contains(n)) {
        return n
      }
      st.add(n)
    }
    return 0
  }
}
