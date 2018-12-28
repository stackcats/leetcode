// https://leetcode.com/problems/complete-binary-tree-inserter/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

typedef struct TreeNode TreeNode;

typedef struct {
  TreeNode **arr;
  int size;
} CBTInserter;

#define MAX_SIZE 11000
CBTInserter *cBTInserterCreate(TreeNode *root) {
  CBTInserter *cbt = malloc(sizeof(CBTInserter));
  cbt->arr = calloc(MAX_SIZE, sizeof(TreeNode *));
  memset(cbt->arr, 0, sizeof(TreeNode *) * MAX_SIZE);
  cbt->size = 0;
  cbt->arr[cbt->size] = root;
  while (cbt->arr[cbt->size] != NULL) {
    TreeNode *node = cbt->arr[cbt->size];
    cbt->arr[cbt->size * 2 + 1] = node->left;
    cbt->arr[cbt->size * 2 + 2] = node->right;
    cbt->size += 1;
  }
  return cbt;
}

int cBTInserterInsert(CBTInserter *obj, int v) {
  TreeNode *node = malloc(sizeof(TreeNode));
  node->val = v;
  node->left = NULL;
  node->right = NULL;
  obj->arr[obj->size] = node;

  int parnetIndex = (obj->size - 1) / 2;
  if (obj->size % 2 == 1) {
    obj->arr[parnetIndex]->left = node;
  } else {
    obj->arr[parnetIndex]->right = node;
  }

  obj->size += 1;

  return obj->arr[parnetIndex]->val;
}

TreeNode *cBTInserterGet_root(CBTInserter *obj) { return obj->arr[0]; }

void cBTInserterFree(CBTInserter *obj) {
  free(obj->arr);
  free(obj);
}

/**
 * Your CBTInserter struct will be instantiated and called as such:
 * struct CBTInserter* obj = cBTInserterCreate(root);
 * int param_1 = cBTInserterInsert(obj, v);
 * struct TreeNode* param_2 = cBTInserterGet_root(obj);
 * cBTInserterFree(obj);
 */
