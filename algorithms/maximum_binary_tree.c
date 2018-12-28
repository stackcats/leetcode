// https://leetcode.com/problems/maximum-binary-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *helper(int *nums, int i, int j) {
  if (i > j) {
    return NULL;
  }

  int maxIndex = i;
  for (int k = i; k <= j; k++) {
    if (nums[maxIndex] < nums[k]) {
      maxIndex = k;
    }
  }

  struct TreeNode *root = malloc(sizeof(struct TreeNode));
  root->val = nums[maxIndex];
  root->left = helper(nums, i, maxIndex - 1);
  root->right = helper(nums, maxIndex + 1, j);
  return root;
}

struct TreeNode *constructMaximumBinaryTree(int *nums, int numsSize) {
  return helper(nums, 0, numsSize - 1);
}
