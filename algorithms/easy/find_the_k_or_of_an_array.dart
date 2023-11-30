import 'dart:math';

class Solution {
  int findKOr(List<int> nums, int k) {
    int len = 31;
    final mp = <int, int>{};
    for (final num in nums) {
      for (int i = 0; i <= len; i++) {
        final t = pow(2, i).toInt();
        if (num & t == t) {
          mp[i] = (mp[i] ?? 0) + 1;
        }
      }
    }
    int ans = 0;
    for (final e in mp.entries) {
      if (e.value >= k) {
        ans += pow(2, e.key).toInt();
      }
    }
    return ans;
  }
}
