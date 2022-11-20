class Solution {
public:
    int maxSum(vector<vector<int>>& grid) {
      int m = grid.size() - 2, n = grid[0].size() - 2;
      int ans = 0;
      for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
          ans = max(ans, grid[i][j] + grid[i][j+1] + grid[i][j+2] + grid[i+1][j+1] + grid[i+2][j] + grid[i+2][j+1] + grid[i+2][j+2]);
        }
      }
      return ans;
    }
};
