// https://leetcode.com/problems/minimum-path-sum/description/

#define min(a, b) ((a) < (b) ? (a) : (b))
int minPathSum(int **grid, int gridRowSize, int gridColSize) {
  int **d = (int **)malloc(sizeof(int *) * gridRowSize);
  for (int i = 0; i < gridRowSize; i++) {
    d[i] = (int *)malloc(sizeof(int) * gridColSize);
    memset(d[i], 0, sizeof(int) * gridColSize);
  }

  d[0][0] = grid[0][0];
  for (int i = 1; i < gridRowSize; i++) {
    d[i][0] = grid[i][0] + d[i - 1][0];
  }

  for (int j = 1; j < gridColSize; j++) {
    d[0][j] = grid[0][j] + d[0][j - 1];
  }

  for (int i = 1; i < gridRowSize; i++) {
    for (int j = 1; j < gridColSize; j++) {
      d[i][j] = min(d[i - 1][j], d[i][j - 1]) + grid[i][j];
    }
  }

  int res = d[gridRowSize - 1][gridColSize - 1];

  for (int i = 0; i < gridRowSize; i++) {
    free(d[i]);
  }

  free(d);

  return res;
}
