// https://leetcode.com/problems/3sum/description/

int compare_ints(const void *a, const void *b) {
  int arg1 = *(const int *)a;
  int arg2 = *(const int *)b;

  return (arg1 > arg2) - (arg1 < arg2);
}

/**
 * Return an array of arrays of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int **threeSum(int *nums, int numsSize, int *returnSize) {
  qsort(nums, numsSize, sizeof(int), compare_ints);

  int **arr = calloc(17000, sizeof(int *));

  for (int i = 0; i < numsSize - 2; i++) {
    if (nums[i] > 0) { // nums[i]大于0 直接退出
      break;
    }

    if (i > 0 && nums[i] == nums[i - 1]) {
      continue;
    }

    int l = i + 1;
    int r = numsSize - 1;
    while (l < r) {
      int sum = nums[i] + nums[l] + nums[r];
      if (sum == 0) {
        arr[*returnSize] = calloc(3, sizeof(int));
        arr[*returnSize][0] = nums[i];
        arr[*returnSize][1] = nums[l];
        arr[*returnSize][2] = nums[r];
        *returnSize += 1;
        int k = 0;
        while (k + l < r && nums[k + l] == nums[l]) {
          k++;
        }
        l += k;

        k = 0;
        while (l < r - k && nums[r - k] == nums[r]) {
          k++;
        }
        r -= k;
      } else if (sum > 0) {
        r--;
      } else {
        l++;
      }
    }
  }

  return arr;
}
