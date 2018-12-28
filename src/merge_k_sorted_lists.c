// https://leetcode.com/problems/merge-k-sorted-lists/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *merge(struct ListNode *l1, struct ListNode *l2) {
  struct ListNode *head = NULL;
  struct ListNode *tail = NULL;
  while (l1 && l2) {
    struct ListNode *node;
    if (l1->val < l2->val) {
      node = l1;
      l1 = l1->next;
    } else {
      node = l2;
      l2 = l2->next;
    }

    if (head == NULL) {
      head = node;
    } else {
      tail->next = node;
    }

    tail = node;
  }

  if (l1) {
    if (head == NULL) {
      head = l1;
    } else {
      tail->next = l1;
    }
  }

  if (l2) {
    if (head == NULL) {
      head = l2;
    } else {
      tail->next = l2;
    }
  }

  return head;
}

// 两两合并
struct ListNode *mergeKLists(struct ListNode **lists, int listsSize) {
  if (listsSize == 0) {
    return NULL;
  }

  if (listsSize == 1) {
    return lists[0];
  }

  for (int i = 0, j = listsSize - 1; i < j; i++, j--) {
    lists[i] = merge(lists[i], lists[j]);
  }

  listsSize = ceil(listsSize / 2.0);

  return mergeKLists(lists, listsSize);
}
