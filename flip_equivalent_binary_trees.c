// https://leetcode.com/problems/flip-equivalent-binary-trees/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
bool flipEquiv(struct TreeNode *root1, struct TreeNode *root2) {
  if (!root1 && !root2) {
    return true;
  }

  if (!root1 || !root2) {
    return false;
  }

  if (root1->val != root2->val) {
    return false;
  }
  int a = flipEquiv(root1->left, root2->left) &&
    flipEquiv(root1->right, root2->right);
  int b = flipEquiv(root1->left, root2->right) &&
    flipEquiv(root1->right, root2->left);

  return a || b;
}
