// https://leetcode.com/problems/linked-list-components/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

int compare_ints(const void *a, const void *b) {
  int arg1 = *(const int *)a;
  int arg2 = *(const int *)b;

  return (arg1 > arg2) - (arg1 < arg2);
}

int numComponents(struct ListNode *head, int *G, int GSize) {
  int hash[10004] = {0};

  for (int i = 0; i < GSize; i++) {
    // O(n)
    hash[G[i]] = 1;
  }

  int res = 0;

  int flag = 0;
  while (head) {
    // O(n)
    while (head && hash[head->val]) {
      head = head->next;
      flag = 1;
    }

    if (flag) {
      res++;
      flag = 0;
    }

    if (head == NULL) {
      break;
    }

    head = head->next;
  }

  // O(n)
  return res;
}
