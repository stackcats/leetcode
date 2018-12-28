// https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/

// 中序遍历
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
typedef struct ListNode_ {
  void *data;
  struct ListNode_ *next;
} ListNode;

typedef struct {
  ListNode *head;
} List;

List *list_init() {
  List *lst = malloc(sizeof(List));
  lst->head = NULL;
  return lst;
}

void list_free(List *lst) {
  while (lst->head) {
    ListNode *node = lst->head;
    lst->head = lst->head->next;
    free(node);
  }
  free(lst);
}

void list_ins_head(List *lst, void *data) {
  ListNode *node = malloc(sizeof(ListNode));
  node->data = data;
  node->next = lst->head;
  lst->head = node;
}

void *list_rm_head(List *lst) {
  ListNode *node = lst->head;
  lst->head = lst->head->next;
  void *data = node->data;
  free(node);
  return data;
}

int kthSmallest(struct TreeNode *root, int k) {
  List *queue = list_init();

  struct TreeNode *iter = root;
  while (1) {
    if (iter) {
      list_ins_head(queue, iter);
      iter = iter->left;
    } else {
      iter = list_rm_head(queue);
      k--;
      if (k == 0) {
        break;
      }
      iter = iter->right;
    }
  }

  free(queue);
  return iter->val;
}
