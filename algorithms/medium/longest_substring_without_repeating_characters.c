// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

// 枚举的时间复杂度为O(n^2)
// 使用下标l, r分别记录子串的两端

// 假设程序当前状态 l = 0, r = 6

// 0 1 2 3 4 5 6 7 8 9
//             r
// a b c d e f d g h i
// l

// 当r移动到6时, 发现d已经在3的位置上出现过，此时最长不重复字串abcdef, 长度为6
// 如果是枚举解法则l，r更新成1, 重新遍历寻找解
// 但是当r运行到6时又产生了重复的字串, 此时最长不重复字串bcdef,
// 长度为5，还没有之前的长 所以枚举会产生大量的重复计算

// 我们可以用hash记录r已经出现过的位置用来更新l避免重复计算,实现O(n)的时间复杂度

// 假设程序当前状态 l = 0, r = 6

// 0 1 2 3 4 5 6 7 8 9
//             r
// a b c d e f d g h i
// l

// 当r移动到6时, 发现d已经在3的位置上出现过，此时可以直接将l更新为4
int lengthOfLongestSubstring(char *s) {
  int max = 0;

  // 使用int[128]数组做hash记录l下一次应该更新的位置
  int hash[128] = {0};

  int l = 0;

  for (int r = 0; s[r] != '\0'; r++) { // O(n)
    int ch = s[r];

    if (l <= hash[ch]) {
      l = hash[ch];
    }

    hash[ch] = r + 1;

    int len = r - l + 1;
    if (max < len) {
      max = len;
    }
  }

  return max;
}
