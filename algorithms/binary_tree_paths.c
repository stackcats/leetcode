// https://leetcode.com/problems/binary-tree-paths/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
char **binaryTreePaths(struct TreeNode *root, int *returnSize) {
  if (!root) {
    *returnSize = 0;
    return NULL;
  }

  char **res;
  if (!root->left && !root->right) {
    res = calloc(1, sizeof(char *));
    res[0] = calloc(16, sizeof(char));
    sprintf(res[0], "%d", root->val);
    *returnSize = 1;
    return res;
  }

  int leftSize = 0, rightSize = 0;
  char **left = binaryTreePaths(root->left, &leftSize);
  char **right = binaryTreePaths(root->right, &rightSize);

  res = calloc(leftSize + rightSize, sizeof(char *));
  int r = 0;
  for (int i = 0; i < leftSize; i++) {
    res[r] = calloc(strlen(left[i]) + 16, sizeof(char));
    sprintf(res[r++], "%d->%s", root->val, left[i]);
    free(left[i]);
  }

  for (int i = 0; i < rightSize; i++) {
    res[r] = calloc(strlen(right[i]) + 16, sizeof(char));
    sprintf(res[r++], "%d->%s", root->val, right[i]);
    free(right[i]);
  }

  free(left);
  free(right);
  *returnSize = leftSize + rightSize;
  return res;
}
