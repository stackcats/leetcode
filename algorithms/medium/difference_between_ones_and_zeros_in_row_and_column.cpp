class Solution {
public:
  vector<vector<int>> onesMinusZeros(vector<vector<int>> &grid) {
    int m = grid.size(), n = grid[0].size();
    vector<int> row_sums(m), col_sums(n);
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        row_sums[i] += grid[i][j];
        col_sums[j] += grid[i][j];
      }
    }

    vector<vector<int>> ans(m, vector<int>(n));
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        ans[i][j] = row_sums[i] * 2 + col_sums[j] * 2 - m - n;
      }
    }
    return ans;
  }
};
