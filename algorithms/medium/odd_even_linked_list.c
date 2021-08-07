// https://leetcode.com/problems/odd-even-linked-list/description/

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

struct ListNode *oddEvenList(struct ListNode *head) {
  List *odd = list_init();
  List *even = list_init();

  bool flag = true;

  while (head) {
    struct ListNode *tmp = head;
    head = head->next;
    tmp->next = NULL;
    if (flag) {
      odd = list_append(odd, tmp);
    } else {
      even = list_append(even, tmp);
    }

    flag = !flag;
  }

  odd = list_append(odd, even->head);
  struct ListNode *res = odd->head;
  free(odd);
  free(even);
  return res;
}
