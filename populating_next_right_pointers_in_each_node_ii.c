// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/description/

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
  while (uncle) {
    if (uncle->left) {
      cousin = uncle->left;
      break;
    }
    if (uncle->right) {
      cousin = uncle->right;
      break;
    }
    uncle = uncle->next;
  }

  if (root->left && root->right) {
    root->left->next = root->right;
    root->right->next = cousin;
  } else if (root->left) {
    root->left->next = cousin;
  } else if (root->right) {
    root->right->next = cousin;
  }

  // 注意处理顺序 必须先处理右子树
  // 如果先处理左子树 右子树的next可能还没有链接
  // 循环uncle查找cousin时就会提前退出
  connect(root->right);
  connect(root->left);
}
