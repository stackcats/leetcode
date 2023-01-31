class Solution {
public:
    int bestTeamScore(vector<int>& scores, vector<int>& ages) {
      vector<pair<int, int>> teams;
      for (int i = 0; i < scores.size(); i++) {
        teams.push_back({ages[i], scores[i]});
      }
      sort(teams.begin(), teams.end(), greater<pair<int, int>>());
      vector<int> dp(teams.size());
      int ans = 0;
      for (int i = 0; i < teams.size(); i++) {
        dp[i] = teams[i].second;
        for (int j = 0; j < i; j++) {
          if (teams[j].second >= teams[i].second) {
            dp[i] = max(dp[i], dp[j] + teams[i].second);
          }
        }
        ans = max(ans, dp[i]);
      }
      return ans;
    }
};
