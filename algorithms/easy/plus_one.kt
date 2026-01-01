class Solution {
  fun plusOne(digits: IntArray): IntArray {
    for (i in digits.size - 1 downTo 0) {
      if (digits[i] < 9) {
        digits[i] += 1
        return digits
      }
      digits[i] = 0
    }

    var ans = IntArray(digits.size + 1)
    ans[0] = 1

    return ans
  }
}
