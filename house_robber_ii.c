// https://leetcode.com/problems/house-robber-ii/description/

// 思路和House Robber一样
// 区别是nums现在为环形 首尾不能同时取
// 将数据分为2组
// 1: [0, numsSize-1) 取第一个元素 不取最后一个
// 2: [1, numsSize)   不取第一个元素 取最后一个
// 两组数据分别运用House Robber的求解过程 取最大的结果为解

#define max(a, b) ((a) < (b) ? (b) : (a))

int dp(int *nums, int start, int end) {
  int len = end - start;
  int *d = (int *)malloc(sizeof(int) * len);
  d[0] = nums[start];
  d[1] = max(nums[start], nums[start + 1]);
  for (int i = 2; i < len; i++) {
    d[i] = max(nums[start + i] + d[i - 2], d[i - 1]);
  }

  int res = d[len - 1];
  free(d);
  return res;
}

int rob(int *nums, int numsSize) {
  if (numsSize <= 0) {
    return 0;
  }
  if (numsSize == 1) {
    return nums[0];
  }
  int res = max(dp(nums, 0, numsSize - 1), dp(nums, 1, numsSize));
  return res;
}
