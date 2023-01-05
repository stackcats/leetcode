class Solution {
public:
  int integerBreak(int n) {
    vector<int> dp(59, 0);
    int ans = 1;
    dp[1] = 1;
    for (int i = 2; i <= n; i++) {
      for (int j = 1; j <= i / 2; j++) {
        array<int, 4> arr{
          dp[j] * dp[i - j],
          j * dp[i - j],
          dp[j] * (i - j),
          j * (i - j),
        };
        int tmp = *max_element(arr.begin(), arr.end());
        dp[i] = max(dp[i], tmp);
      }
      ans = max(ans, dp[i]);
    }
    return ans;
  }
};
