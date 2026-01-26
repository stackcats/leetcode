class Solution {
  fun minimumAbsDifference(arr: IntArray): List<List<Int>> {
    arr.sort()

    val ans = mutableListOf<List<Int>>()
    var mi = Int.MAX_VALUE

    for (i in 1..<arr.size) {
      val diff = arr[i] - arr[i - 1]
      if (diff < mi) {
        mi = diff
        ans.clear()
        ans.add(listOf(arr[i - 1], arr[i]))
      } else if (diff == mi) {
        ans.add(listOf(arr[i - 1], arr[i]))
      }
    }
    return ans
  }
}
