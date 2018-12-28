// https://leetcode.com/problems/range-sum-query-immutable/description/

typedef struct {
  int *d;
} NumArray;

NumArray *numArrayCreate(int *nums, int numsSize) {
  int *d = calloc(numsSize, sizeof(int));
  for (int i = 0; i < numsSize; i++) {
    if (i == 0) {
      d[0] = nums[0];
    } else {
      d[i] = d[i - 1] + nums[i];
    }
  }
  NumArray *na = malloc(sizeof(NumArray));
  na->d = d;
  return na;
}

int numArraySumRange(NumArray *obj, int i, int j) {
  // sumRange(i, j) = sumRange(0, j) - sumRange(0, i - 1)
  if (i == 0)
    return obj->d[j];
  return obj->d[j] - obj->d[i - 1];
}

void numArrayFree(NumArray *obj) {
  free(obj->d);
  free(obj);
}

/**
 * Your NumArray struct will be instantiated and called as such:
 * struct NumArray* obj = numArrayCreate(nums, numsSize);
 * int param_1 = numArraySumRange(obj, i, j);
 * numArrayFree(obj);
 */
