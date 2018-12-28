// https://leetcode.com/problems/partition-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
typedef struct {
  struct ListNode *head;
  struct ListNode *tail;
} List;

List *list_init() {
  List *l = (List *)malloc(sizeof(List));
  l->head = NULL;
  l->tail = NULL;
  return l;
}

List *list_append(List *l, struct ListNode *node) {
  if (l->head == NULL) {
    l->head = node;
  } else {
    l->tail->next = node;
  }

  l->tail = node;

  return l;
}

struct ListNode *partition(struct ListNode *head, int x) {
  List *less = list_init();
  List *more = list_init();

  while (head) {
    struct ListNode *tmp = head;
    head = head->next;
    tmp->next = NULL;
    if (tmp->val < x) {
      less = list_append(less, tmp);
    } else {
      more = list_append(more, tmp);
    }
  }

  less = list_append(less, more->head);
  struct ListNode *res = less->head;
  free(less);
  free(more);
  return res;
}
