// https://leetcode.com/problems/rotate-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

int listLen(struct ListNode *lst) {
  int ct = 0;
  while (lst) {
    ct++;
    lst = lst->next;
  }
  return ct;
}

struct ListNode *rotateRight(struct ListNode *lst, int k) {
  if (lst == NULL) {
    return NULL;
  }

  struct ListNode *head = lst;
  struct ListNode *tail = NULL;
  struct ListNode *pre = NULL;

  struct ListNode *iter = lst;

  k %= listLen(lst);

  while (k-- > 0) {
    tail = iter;
    iter = iter->next;
  }

  while (tail && tail->next) {
    tail = tail->next;
    pre = head;
    head = head->next;
  }

  if (pre == NULL) {
    return lst;
  }

  pre->next = NULL;
  tail->next = lst;

  return head;
}
