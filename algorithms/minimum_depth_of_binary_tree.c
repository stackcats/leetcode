/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
typedef struct TreeNode TreeNode;

#define min(a, b) ((a) < (b) ? (a) : (b))

void helper(TreeNode *root, int dp, int *m) {
  if (!root) {
    return;
  }

  if (!root->left && !root->right) {
    *m = min(*m, dp); // 只有叶子节点计算最小深度
    return;
  }

  helper(root->left, dp + 1, m);
  helper(root->right, dp + 1, m);
}

int minDepth(struct TreeNode *root) {
  if (!root) {
    return 0;
  }

  int m = INT_MAX; // 最小深度
  int dp = 1;      // 当前深度
  helper(root, dp, &m);
  return m;
}
