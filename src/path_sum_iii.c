/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int helper(struct TreeNode *root, int sum) {
  if (!root) {
    return 0;
  }

  int left = helper(root->left, sum - root->val);
  int right = helper(root->right, sum - root->val);
  return root->val == sum ? left + right + 1 : left + right;
}

int pathSum(struct TreeNode *root, int sum) {
  if (!root) {
    return 0;
  }

  return helper(root, sum) + pathSum(root->left, sum) +
    pathSum(root->right, sum);
}
