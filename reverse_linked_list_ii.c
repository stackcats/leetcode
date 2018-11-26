// https://leetcode.com/problems/reverse-linked-list-ii/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *reverseBetween(struct ListNode *lst, int m, int n) {
  struct ListNode *iter = lst;
  struct ListNode *head = NULL;
  struct ListNode *tail = NULL;
  struct ListNode *penult = NULL; // 记录m-1的位置

  while (iter && m > 0) {
    struct ListNode *node = iter;
    iter = iter->next;

    if (head == NULL) {
      head = node;
      tail = node;
    } else {
      tail->next = node;
      penult = tail;
      tail = node;
    }

    tail->next = NULL; // 避免产生环

    m--;
    n--;
  }

  while (iter && n > 0) {
    struct ListNode *node = iter;

    iter = iter->next;

    if (penult == NULL) {
      node->next = head;
      head = node;
    } else {
      node->next = penult->next;
      penult->next = node;
    }
    n--;
  }

  if (iter) {
    tail->next = iter;
  }

  // O(n)
  return head;
}
