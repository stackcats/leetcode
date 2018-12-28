// https://leetcode.com/problems/split-linked-list-in-parts/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int len(struct ListNode *l) {
  int ct = 0;
  while (l) {
    ct++;
    l = l->next;
  }

  return ct;
}

struct ListNode **splitListToParts(struct ListNode *root, int k,
                                   int *returnSize) {

  *returnSize = k;
  struct ListNode **arr =
    (struct ListNode **)malloc(sizeof(struct ListNode *) * k);

  for (int i = 0; i < k; i++) {
    arr[i] = NULL;
  }

  int ct = len(root);
  int rem = ct % k;

  struct ListNode *iter = root;
  for (int i = 0; i < k && iter; i++) {
    int size = ct / k;
    if (rem > 0) {
      size++;
      rem--;
    }

    struct ListNode *tail = NULL;
    while (size-- > 0 && iter) {
      struct ListNode *node = iter;
      iter = iter->next;
      node->next = NULL;

      if (arr[i] == NULL) {
        arr[i] = node;
      } else {
        tail->next = node;
      }
      tail = node;
    }
  }

  return arr;
}
