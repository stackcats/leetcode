// https://leetcode.com/problems/house-robber/description/
// 动态规划
// 初始状态
// d[0] = nums[0]
// d[1] = max(d[0], d[1])
// 状态转移方程
// d[i] = max(nums[i] + d[i-2], d[i-1])
#define max(a, b) ((a) < (b) ? (b) : (a))

int rob(int *nums, int numsSize) {
  if (numsSize == 0) {
    return 0;
  }
  if (numsSize == 1) {
    return nums[0];
  }

  int *d = (int *)malloc(sizeof(int) * numsSize);
  d[0] = nums[0];
  d[1] = max(nums[0], nums[1]);
  for (int i = 2; i < numsSize; i++) {
    d[i] = max(nums[i] + d[i - 2], d[i - 1]);
  }

  int res = d[numsSize - 1];
  free(d);
  return res;
}
