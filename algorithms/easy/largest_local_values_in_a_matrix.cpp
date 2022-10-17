class Solution {
public:
    vector<vector<int>> largestLocal(vector<vector<int>>& grid) {
      const int n = grid.size() - 2;
      vector<vector<int>> ans(n, vector<int>(n));
      for (auto i = 0; i < n; i++) {
        for (auto j = 0; j < n; j++) {
          for (auto di = 0; di < 3; di++) {
            for (auto dj = 0; dj < 3; dj++)
              ans[i][j] = max(ans[i][j], ans[i+di][j+dj]);
          }
        }
      }
      return ans;
    }
};
