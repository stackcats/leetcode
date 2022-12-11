class Solution {
public:
    int deleteGreatestValue(vector<vector<int>>& grid) {
      for (int i = 0; i < grid.size(); ++i) {
        sort(grid[i].begin(), grid[i].end());
      }
      int ans = 0;
      for (int j = 0; j < grid[0].size(); ++j) {
        int tmp = 0;
        for (int i = 0; i < grid.size(); ++i) {
          tmp = max(tmp, grid[i][j]);
        }
        ans += tmp;
      }
      return ans;
    }
};
