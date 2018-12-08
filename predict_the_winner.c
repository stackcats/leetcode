// https://leetcode.com/problems/predict-the-winner/description/

// 和stone game一样
inline max(int a, int b) { return a < b ? b : a; }

bool PredictTheWinner(int *nums, int numsSize) {
  int **d = calloc(numsSize, sizeof(int *));
  for (int i = 0; i < numsSize; i++) {
    d[i] = calloc(numsSize, sizeof(int));
    d[i][i] = nums[i];
  }

  for (int len = 1; len < numsSize; len++) {
    for (int i = 0; i < numsSize - len; i++) {
      d[i][i + len] =
          max(nums[i] - d[i + 1][i + len], nums[i + len] - d[i][i + len - 1]);
    }
  }
  return d[0][numsSize - 1] >= 0;
}
