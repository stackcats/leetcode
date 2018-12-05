// https://leetcode.com/problems/happy-number/description/

typedef struct Node_ {
  int val;
  struct Node_ *next;
} Node;

typedef struct {
  Node *head;
} List;

List *list_init() {
  List *lst = malloc(sizeof(List));
  lst->head = NULL;
  return lst;
}

void list_ins_head(List *lst, int val) {
  Node *node = malloc(sizeof(Node));
  node->val = val;
  node->next = lst->head;
  lst->head = node;
}

int list_lookup(List *lst, int val) {
  Node *iter = lst->head;
  while (iter) {
    if (iter->val == val) {
      return 1;
    }
    iter = iter->next;
  }

  return 0;
}

void list_free(List *lst) {
  while (lst->head) {
    Node *node = lst->head;
    lst->head = lst->head->next;
    free(node);
  }

  free(lst);
}

int sumOfSq(int n) {
  int ans = 0;
  while (n > 0) {
    int d = n % 10;
    ans += d * d;
    n /= 10;
  }
  return ans;
}

bool isHappy(int n) {
  List *lst = list_init();

  while (n != 1) {
    if (list_lookup(lst, n)) {
      return 0;
    }
    list_ins_head(lst, n);
    n = sumOfSq(n);
  }

  list_free(lst);
  return 1;
}
