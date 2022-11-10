class Solution {
public:
  int countDistinctIntegers(vector<int> &nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    for (auto n : nums) {
      int m = 0;
      while (n > 0) {
        m = m * 10 + n % 10;
        n /= 10;
      }
      s.insert(m);
    }
    return s.size();
  }
};
