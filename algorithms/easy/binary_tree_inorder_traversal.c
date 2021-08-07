// https://leetcode.com/problems/binary-tree-inorder-traversal/description/

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

typedef struct SNode_ {
  void *data;
  struct SNode_ *next;
} SNode;

SNode *snode_init(void *data) {
  SNode *node = (SNode *)malloc(sizeof(SNode));
  node->data = data;
  node->next = NULL;
  return node;
}

typedef struct {
  SNode *base;
  int size;
} Stack;

Stack *stack_init() {
  Stack *st = (Stack *)malloc(sizeof(Stack));
  st->base = NULL;
  st->size = 0;
  return st;
}

#define stack_is_empty(st) ((st)->base == NULL)

void stack_free(Stack *st) {
  if (!st) {
    return;
  }

  while (st->base) {
    SNode *n = st->base->data;
    st->base = st->base->next;
    free(n);
  }

  free(st);
}

void stack_push(Stack *st, void *data) {
  assert(st);

  SNode *node = snode_init(data);

  if (st->base) {
    node->next = st->base;
  }

  st->base = node;

  st->size += 1;
}

void *stack_pop(Stack *st) {
  assert(st && st->base);

  SNode *node = st->base;
  st->base = st->base->next;

  st->size -= 1;

  void *data = node->data;
  free(node);
  return data;
}

int *inorderTraversal(struct TreeNode *root, int *returnSize) {
  struct TreeNode *iter = root;

  Stack *st = stack_init();
  Stack *res = stack_init();
  int done = 0;

  while (!done) {
    if (iter) {
      stack_push(st, iter);
      iter = iter->left;
    } else {
      if (stack_is_empty(st)) {
        done = 1;
      } else {
        iter = stack_pop(st);
        stack_push(res, iter);
        iter = iter->right;
      }
    }
  }

  free(st);

  *returnSize = res->size;
  int *arr = (int *)malloc(sizeof(int) * res->size);
  int i = *returnSize - 1;
  while (!stack_is_empty(res)) {
    struct TreeNode *node = stack_pop(res);
    arr[i--] = node->val;
  }

  free(res);

  return arr;
}
