class Solution {
  int sumCounts(List<int> nums) {
    int ans = 0;
    for (int i = 0; i < nums.length; i++) {
      for (int j = i; j < nums.length; j++) {
        final st = nums.sublist(i, j + 1).toSet();
        ans += st.length * st.length;
      }
    }
    return ans;
  }
}
