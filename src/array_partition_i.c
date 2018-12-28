// https://leetcode.com/problems/array-partition-i/description/

int compare_ints(const void *a, const void *b) {
  int arg1 = *(const int *)a;
  int arg2 = *(const int *)b;
  return (arg1 > arg2) - (arg1 < arg2);
}

int arrayPairSum(int *nums, int numsSize) {
  qsort(nums, numsSize, sizeof(int), compare_ints);
  int sum = 0;
  for (int i = 0; i < numsSize; i += 2) {
    sum += nums[i];
  }
  return sum;
}
