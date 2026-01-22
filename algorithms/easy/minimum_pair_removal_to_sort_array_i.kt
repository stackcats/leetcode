class Solution {
  fun isSorted(nums: MutableList<Int>): Boolean {
    for (i in 1..<nums.size) {
      if (nums[i] < nums[i - 1]) {
        return false
      }
    }
    return true
  }

  fun minimumPairRemoval(nums: IntArray): Int {
    val arr = nums.toMutableList()
    var ct = 0

    while (!isSorted(arr)) {
      var minSum = Int.MAX_VALUE
      var minIndex = 0
      for (i in 1..<arr.size) {
        val sum = arr[i - 1] + arr[i]
        if (sum < minSum) {
          minSum = sum
          minIndex = i - 1
        }
      }
      arr[minIndex] = minSum
      arr.removeAt(minIndex + 1)
      ct += 1
    }

    return ct
  }
}
