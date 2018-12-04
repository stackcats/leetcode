// https://leetcode.com/problems/min-cost-climbing-stairs/description/

inline int min(a, b) { return a < b ? a : b; }

int minCostClimbingStairs(int *cost, int costSize) {
  int *d = calloc(costSize, sizeof(int));

  d[0] = cost[0];
  d[1] = cost[1];

  for (int i = 2; i < costSize; i++) {
    d[i] = min(d[i - 1], d[i - 2]) + cost[i];
  }

  int ans = min(d[costSize - 1], d[costSize - 2]);
  free(d);
  return ans;
}
