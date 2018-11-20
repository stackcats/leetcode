// https://leetcode.com/problems/maximum-subarray/description/

// 动态规划

#define max(a, b) ((a) > (b) ? (a) : (b))

int maxSubArray(int *nums, int numsSize) {
  int *mem = (int *)malloc(sizeof(int) * numsSize);
  mem[0] = nums[0];

  int res = INT_MIN;
  for (int i = 0; i < numsSize; i++) {
    // O(n)
    mem[i] = max(nums[i], mem[i - 1] + nums[i]);
    res = max(mem[i], res);
  }

  free(mem);

  // O(n)
  return res;
}
