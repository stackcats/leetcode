import 'dart:math';

class Solution {
  int distributeCandies(int n, int limit) {
    int ct = 0;
    for (int i = min(n, limit); i >= 0; i--) {
      for (int j = min(n - i, limit); j >= 0; j--) {
        if (n - i - j > limit) {
          break;
        }
        ct++;
      }
    }
    return ct;
  }
}
