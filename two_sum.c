// https://leetcode.com/problems/two-sum

// 先排序再线性搜索
// 使用Node结构记录排序前的下标
typedef struct {
  int val;
  int index;
} Node;

int compare_nodes(const void *a, const void *b) {
  const Node *arg1 = (const Node *)a;
  const Node *arg2 = (const Node *)b;

  // arg1-val - arg2->val 的形式可能造成溢出
  return (arg1->val > arg2->val) - (arg1->val < arg2->val);
}

int *twoSum(int *nums, int numsSize, int target) {
  Node *arr = (Node *)malloc(sizeof(Node) * numsSize);
  for (int i = 0; i < numsSize; i++) {
    // O(n)
    arr[i] = (Node){.val = nums[i], .index = i};
  }

  qsort(arr, numsSize, sizeof(Node), compare_nodes); // O(nlog(n))

  int i = 0;
  int j = numsSize - 1;
  int *res = (int *)malloc(sizeof(int) * 2);
  while (i < j) {
    // O(n)
    int sum = arr[i].val + arr[j].val;
    if (sum == target) {
      res[0] = arr[i].index;
      res[1] = arr[j].index;
      break;
    } else if (sum < target) {
      i++;
    } else {
      j--;
    }
  }

  free(arr);

  // O(n) + O(nlog(n)) + O(n) = O(nlog(n))
  return res;
}
