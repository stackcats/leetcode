struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

struct TreeNode* h(struct TreeNode* root, int parent) {
  if (root == NULL) {
    return NULL;
  }
  root->right = h(root->right, parent);
  if (root->right) {
    struct TreeNode* cur = root->right;
    while (cur->left) {
      cur = cur->left;
    }
    root->val += cur->val;
        
  } else {
    root->val += parent;
  }
  root->left = h(root->left, root->val);
  return root;
}

struct TreeNode* bstToGst(struct TreeNode* root){
  return h(root, 0);
}
