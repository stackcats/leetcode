class Solution {
public:
    int findMaxK(vector<int>& nums) {
      set<int> s;
      int ans = -1;
      for (auto n: nums) {
        s.insert(n);
        if (s.find(-n) != s.end()) {
          ans = max(ans, abs(n));
        }
      }
      return ans;
    }
};
