// https://leetcode.com/problems/leaf-similar-trees/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

typedef struct Node_ {
  void *data;
  struct Node_ *next;
} Node;

Node *node_init(void *data) {
  Node *n = malloc(sizeof(Node));
  n->data = data;
  n->next = NULL;
  return n;
}

typedef struct {
  Node *head;
  Node *tail;
} List;

List *list_init() {
  List *lst = malloc(sizeof(List));
  lst->head = NULL;
  lst->tail = NULL;
  return lst;
}

void list_free(List *lst) {
  while (lst->head) {
    Node *n = lst->head;
    lst = lst->head->next;
    free(n);
  }
  free(lst);
}

void list_ins_head(List *lst, void *data) {
  Node *node = node_init(data);
  if (lst->head == NULL) {
    lst->tail = node;
  } else {
    node->next = lst->head;
  }
  lst->head = node;
}

void *list_rm_head(List *lst) {
  Node *node = lst->head;
  lst->head = node->next;
  if (lst->head == NULL) {
    lst->tail = NULL;
  }

  void *data = node->data;
  free(node);
  return data;
}

List *leaves(struct TreeNode *root) {
  List *st = list_init();
  List *ls = list_init();
  list_ins_head(st, root);
  while (st->head) {
    struct TreeNode *t = list_rm_head(st);
    if (!t->left && !t->right) {
      list_ins_head(ls, t);
    }

    if (t->right) {
      list_ins_head(st, t->right);
    }

    if (t->left) {
      list_ins_head(st, t->left);
    }
  }

  free(st);
  return ls;
}

bool leafSimilar(struct TreeNode *root1, struct TreeNode *root2) {
  List *ls1 = leaves(root1);
  List *ls2 = leaves(root2);
  Node *i = ls1->head;
  Node *j = ls2->head;

  int res = 1;
  while (i && j) {
    struct TreeNode *a = i->data;
    struct TreeNode *b = j->data;
    if (a->val != b->val) {
      break;
    }
    i = i->next;
    j = j->next;
  }

  if (i || j) {
    res = 0;
  }

  free(ls1);
  free(ls2);
  return res;
}
