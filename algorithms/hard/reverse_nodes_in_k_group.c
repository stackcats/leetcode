// https://leetcode.com/problems/reverse-nodes-in-k-group/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode *reverseKGroup(struct ListNode *lst, int k) {
  struct ListNode *pre = NULL;
  struct ListNode *curr = lst;
  while (curr) {
    int n = 0;
    struct ListNode *head = NULL;
    struct ListNode *tail = NULL;
    while (curr && n < k) {
      struct ListNode *tmp = curr;
      curr = curr->next;
      n++;
      if (head == NULL) {
        head = tail = tmp;
        tmp->next = NULL;
      } else {
        tmp->next = head;
        head = tmp;
      }
    }

    if (n < k) {
      // 不足k个时不翻转
      // 将已经翻转的部分再还原
      struct ListNode *x = NULL;
      struct ListNode *y = NULL;
      while (head) {
        struct ListNode *tmp = head;
        head = head->next;
        if (x == NULL) {
          x = y = tmp;
        } else {
          tmp->next = x;
          x = tmp;
        }
      }
      head = x;
      tail = y;
    }

    if (pre == NULL) {
      lst = head;
    } else {
      pre->next = head;
    }

    pre = tail;
    tail->next = curr;
  }

  return lst;
}
