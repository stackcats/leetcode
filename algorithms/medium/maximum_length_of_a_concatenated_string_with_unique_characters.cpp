class Solution {
public:
    int maxLength(vector<string>& arr) {
      vector<bitset<26>> dp = { bitset<26>() };
      size_t ans = 0;
      for (string& s : arr) {
        bitset<26> a;
        for (char c : s) {
          a.set(c - 'a');
        }
        if (a.count() < s.size()) {
          continue;
        }
        for (int i = dp.size() - 1; i >= 0; --i) {
          if ((dp[i] & a).any()) {
            continue;
          }
          dp.push_back(dp[i] | a);
          ans = max(ans, (dp[i] | a).count());
        }
      }
      return (int)ans;
    }
};
