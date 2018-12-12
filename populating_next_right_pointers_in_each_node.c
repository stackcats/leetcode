// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/description/

/**
 * Definition for binary tree with next pointer.
 * struct TreeLinkNode {
 *  int val;
 *  struct TreeLinkNode *left, *right, *next;
 * };
 *
 */
void connect(struct TreeLinkNode *root) {
  if (!root) {
    return;
  }

  struct TreeLinkNode *uncle = root->next;
  struct TreeLinkNode *cousin = NULL;
  if (uncle) {
    // 满二叉树不需要和(II)一样循环查找
    cousin = uncle->left ? uncle->left : uncle->right;
  }

  if (root->left && root->right) {
    root->left->next = root->right;
    root->right->next = cousin;
  } else if (root->left) {
    root->left->next = cousin;
  } else if (root->right) {
    root->right->next = cousin;
  }

  connect(root->right);
  connect(root->left);
}
