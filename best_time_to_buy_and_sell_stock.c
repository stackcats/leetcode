// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/

int maxProfit(int *prices, int pricesSize) {
  int lowest = INT_MAX; // 记录当前遇见过的最小值
  int max_profit = 0;
  for (int i = 0; i < pricesSize; i++) {
    if (lowest > prices[i]) {
      lowest = prices[i];
    } else {
      // 比最小值大时计算利润
      int p = prices[i] - lowest;
      if (max_profit < p) {
        max_profit = p;
      }
    }
  }

  return max_profit;
}
