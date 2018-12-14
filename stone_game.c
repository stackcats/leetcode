// https://leetcode.com/problems/stone-game/description/

inline int max(int a, int b) { return a < b ? b : a; }

// 用二维数组缓存数据把决策树递归转化为循环
bool stoneGame(int *piles, int pilesSize) {
  int **d = calloc(pilesSize, sizeof(int *));
  for (int i = 0; i < pilesSize; i++) {
    d[i] = calloc(pilesSize, sizeof(int));
    d[i][i] = piles[i];
  }

  for (int len = 1; len < pilesSize; len++) {
    for (int i = 0; i < pilesSize - len; i++) {
      d[i][i + len] =
          max(piles[i] - d[i + 1][i + len], piles[i + len] - d[i][i + len - 1]);
    }
  }
  return d[0][pilesSize - 1] > 0;
}
