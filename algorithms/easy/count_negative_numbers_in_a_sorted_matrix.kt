class Solution {
  fun countNegatives(grid: Array<IntArray>): Int =
      grid.sumOf {
        var l = 0
        var r = it.size - 1
        while (l <= r) {
          var m = l + (r - l) / 2
          if (it[m] >= 0) {
            l = m + 1
          } else {
            r = m - 1
          }
        }
        it.size - l
      }
}
