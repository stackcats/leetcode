class Solution {
  fun minimumBoxes(apple: IntArray, capacity: IntArray): Int {
    var totalApples = apple.sum()
    var ct = 0

    capacity.sortDescending()
    for (c in capacity) {
      totalApples -= c
      ct++
      if (totalApples <= 0) {
        break
      }
    }

    return ct
  }
}
