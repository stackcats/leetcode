class Solution {
  fun nextGreatestLetter(letters: CharArray, target: Char): Char {
    var l = 0
    var r = letters.size - 1
    var ans = 0
    while (l <= r) {
      val mid = (l + r) / 2
      if (letters[mid] > target) {
        ans = mid
        r = mid - 1
      } else {
        l = mid + 1
      }
    }

    return letters[ans]
  }
}
