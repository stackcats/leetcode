// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *node_init(int val) {
  struct TreeNode *t = (struct TreeNode *)malloc(sizeof(struct TreeNode));
  t->val = val;
  t->left = NULL;
  t->right = NULL;
  return t;
}

struct TreeNode *sortedArrayToBST_(int *nums, int i, int j) {
  if (i <= j) {
    int mid = i + (j - i) / 2;
    struct TreeNode *t = node_init(nums[mid]);
    t->left = sortedArrayToBST_(nums, i, mid - 1);
    t->right = sortedArrayToBST_(nums, mid + 1, j);
    return t;
  }
  return NULL;
}

struct TreeNode *sortedArrayToBST(int *nums, int numsSize) {
  return sortedArrayToBST_(nums, 0, numsSize - 1);
}
