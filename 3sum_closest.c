// https://leetcode.com/problems/3sum-closest/description/

int compare_ints(const void *a, const void *b) {
  int arg1 = *(const int *)a;
  int arg2 = *(const int *)b;

  return (arg1 > arg2) - (arg1 < arg2);
}

int threeSumClosest(int *nums, int numsSize, int target) {
  qsort(nums, numsSize, sizeof(int), compare_ints);

  int min = INT_MAX;
  int ans = target;
  for (int i = 0; i < numsSize - 2; i++) {
    int l = i + 1;
    int r = numsSize - 1;
    while (l < r) {
      int s = nums[i] + nums[l] + nums[r];
      if (s == target) {
        return s;
      }

      int x = abs(target - s);
      if (min > x) {
        min = x;
        ans = s;
      }
      if (s > target) {
        r--;
      } else {
        l++;
      }
    }
  }

  return ans;
}
