class Solution {
public:
    int maximumCount(vector<int>& nums) {
      int pos{0}, neg{0};
      for (auto n : nums) {
        if (n > 0) {
          pos++;
        } else if (n < 0) {
          neg++;
        }
      }
      return max(pos, neg);
    }
};
