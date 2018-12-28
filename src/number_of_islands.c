// https://leetcode.com/problems/number-of-islands/description/

// DFS

void adjacent(char **grid, int **visited, int x, int y, int row, int col) {
  if (x >= 0 && x < row && y >= 0 && y < col && grid[x][y] == '1' &&
      visited[x][y] == 0) {
    visited[x][y] = 1;
    int dir[4][2] = {{-1, 0}, {0, 1}, {1, 0}, {0, -1}};
    for (int i = 0; i < 4; i++) {
      int dx = dir[i][0] + x;
      int dy = dir[i][1] + y;
      adjacent(grid, visited, dx, dy, row, col);
    }
  }
}

int numIslands(char **grid, int gridRowSize, int gridColSize) {
  int num = 0;

  int **visited = calloc(gridRowSize, sizeof(int *));
  for (int i = 0; i < gridRowSize; i++) {
    visited[i] = calloc(gridColSize, sizeof(int));
  }

  for (int i = 0; i < gridRowSize; i++) {
    for (int j = 0; j < gridColSize; j++) {
      if (grid[i][j] == '1' && visited[i][j] == 0) {
        adjacent(grid, visited, i, j, gridRowSize, gridColSize);
        num++;
      }
    }
  }

  for (int i = 0; i < gridRowSize; i++) {
    free(visited[i]);
  }
  free(visited);
  return num;
}
