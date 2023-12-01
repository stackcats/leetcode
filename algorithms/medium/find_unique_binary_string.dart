import 'dart:math';

class Solution {
  String findDifferentBinaryString(List<String> nums) {
    final st = nums.map((e) => int.parse(e, radix: 2)).toSet();
    int end = pow(2, nums[0].length) as int;
    for (int i = 0; i <= end; i++) {
      if (!st.contains(i)) {
        return i.toRadixString(2).padLeft(nums[0].length, '0');
      }
    }
    return '';
  }
}
