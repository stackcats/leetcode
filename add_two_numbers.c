// https://leetcode.com/problems/add-two-numbers

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *init_node(int val) {
  struct ListNode *node = (struct ListNode *)malloc(sizeof(struct ListNode));
  node->val = val;
  node->next = NULL;
  return node;
}

typedef struct {
  struct ListNode *head;
  struct ListNode *tail;
} List;

List *init_list() {
  List *l = (List *)malloc(sizeof(List));
  l->head = NULL;
  l->tail = NULL;
  return l;
}

List *append(List *l, int val) {
  struct ListNode *node = init_node(val);
  if (l->head == NULL) {
    l->head = node;
    l->tail = node;
  } else {
    l->tail->next = node;
    l->tail = node;
  }
  return l;
}

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2) {
  List *l = init_list();

  int carry = 0;
  while (l1 != NULL && l2 != NULL) {
    int val = l1->val + l2->val + carry;
    carry = val / 10;
    val %= 10;

    append(l, val);

    l1 = l1->next;
    l2 = l2->next;
  }

  while (l1 != NULL) {
    int val = l1->val + carry;
    carry = val / 10;
    val %= 10;

    append(l, val);

    l1 = l1->next;
  }

  while (l2 != NULL) {
    int val = l2->val + carry;
    carry = val / 10;
    val %= 10;

    append(l, val);

    l2 = l2->next;
  }

  if (carry > 0) {
    append(l, carry);
  }

  struct ListNode *res = l->head;
  free(l);

  // O(n), n = max(len(l1), len(l2))
  return res;
}
