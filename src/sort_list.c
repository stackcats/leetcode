// https://leetcode.com/problems/sort-list/description/

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

struct ListNode *mergeList(struct ListNode *l1, struct ListNode *l2) {
  List *l = list_init();

  while (l1 && l2) {
    if (l1->val < l2->val) {
      l = list_append(l, l1);
      l1 = l1->next;
    } else {
      l = list_append(l, l2);
      l2 = l2->next;
    }
  }

  if (l1) {
    l = list_append(l, l1);
  } else if (l2) {
    l = list_append(l, l2);
  }

  struct ListNode *head = l->head;
  free(l);

  return head;
}

struct ListNode *sortList(struct ListNode *head) {
  if (head == NULL || head->next == NULL) {
    return head;
  }

  struct ListNode *p = head;
  struct ListNode *q = head;

  struct ListNode *pre = NULL;
  while (q && q->next) {
    q = q->next->next;
    pre = p;
    p = p->next;
  }

  if (pre) {
    pre->next = NULL;
  }
  return mergeList(sortList(head), sortList(p));
}
