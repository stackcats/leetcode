class Solution {
public:
    int numSquares(int n) {
      vector<int> dp{0};
      while (dp.size() <= n) {
        int x = INT_MAX, m = dp.size();
        for (int i = 1; i * i <= m; ++i) {
          x = min(x, dp[m - i * i] + 1);
        }
        dp.push_back(x);
      }
      return dp.back();
    }
};
