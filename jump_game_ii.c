// https://leetcode.com/problems/jump-game-ii/description/

// d[i] 维护从0到i需要的最少步数
int jump(int *nums, int numsSize) {
  int *d = calloc(numsSize, sizeof(int));
  memset(d, 0, sizeof(int) * numsSize);
  d[0] = 0;
  int j = 1;
  for (int i = 0; i < numsSize; i++) {
    while (nums[i] + i >= j && j < numsSize) {
      d[j] = d[i] + 1;
      j++;
    }
  }
  int ans = d[numsSize - 1];
  free(d);
  return ans;
}
