// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/

int maxProfit(int *prices, int pricesSize) {
  int lowest = INT_MAX;  // 当前最小值
  int highest = INT_MIN; // 当前最大值
  int totalProfit = 0;
  int maxProfit = 0; // 当前最大利润
  for (int i = 0; i < pricesSize; i++) {
    if (highest > prices[i]) {
      // highest > prices[i]时
      // 说明在i时出现了下跌
      // highest为局部峰值 应该在highest卖掉
      totalProfit += maxProfit;
      // 卖掉后当前利润为0
      maxProfit = 0;

      // 重新初始化当前最小值与当前最大值
      lowest = INT_MAX;
      highest = INT_MIN;
    }

    if (lowest > prices[i]) {
      // 价格下跌更新当前最小值
      lowest = prices[i];
    } else {
      // 价格上涨计算当前最大利润和当前最大值
      int p = prices[i] - lowest;
      if (maxProfit < p) {
        highest = prices[i];
        maxProfit = p;
      }
    }
  }

  // 循环中计算totalProfit只判断了出现局部峰值的情况
  // 没有统计在最后一个元素卖出的情况 所以需要加上当前利润
  return totalProfit + maxProfit;
}
