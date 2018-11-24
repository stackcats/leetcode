typedef struct NODE {
  int val;
  struct NODE *next;
} Node;

Node *nodeCreate(int val) {
  Node *n = (Node *)malloc(sizeof(Node));
  n->val = val;
  n->next = NULL;
  return n;
}

typedef struct {
  Node *head;
  Node *tail;
  int size;
} MyLinkedList;

/** Initialize your data structure here. */
MyLinkedList *myLinkedListCreate() {
  MyLinkedList *l = (MyLinkedList *)malloc(sizeof(MyLinkedList));
  l->head = NULL;
  l->tail = NULL;
  l->size = 0;
  return l;
}

/** Get the value of the index-th node in the linked list. If the index is
 * invalid, return -1. */
int myLinkedListGet(MyLinkedList *obj, int index) {
  if (obj == NULL || index < 0 || index >= obj->size) {
    return -1;
  }

  Node *iter = obj->head;

  while (index-- > 0) {
    iter = iter->next;
  }

  return iter->val;
}

/** Add a node of value val before the first element of the linked list. After
 * the insertion, the new node will be the first node of the linked list. */
void myLinkedListAddAtHead(MyLinkedList *obj, int val) {
  Node *node = nodeCreate(val);

  if (obj->head == NULL) {
    obj->tail = node;
  } else {
    node->next = obj->head;
  }

  obj->head = node;
  obj->size += 1;
}

/** Append a node of value val to the last element of the linked list. */
void myLinkedListAddAtTail(MyLinkedList *obj, int val) {
  Node *node = nodeCreate(val);

  if (obj->head == NULL) {
    obj->head = node;
  } else {
    obj->tail->next = node;
  }

  obj->tail = node;
  obj->size += 1;
}

/** Add a node of value val before the index-th node in the linked list. If
 * index equals to the length of linked list, the node will be appended to the
 * end of linked list. If index is greater than the length, the node will not be
 * inserted. */
void myLinkedListAddAtIndex(MyLinkedList *obj, int index, int val) {
  if (obj == NULL || index < 0 || index > obj->size) {
    return;
  }

  Node *curr = obj->head;
  Node *pre = NULL;
  while (index-- > 0) {
    pre = curr;
    curr = curr->next;
  }

  Node *n = nodeCreate(val);

  if (pre == NULL) {
    // pre = NULL 插入的为首节点
    obj->head = n;
  } else {
    n->next = curr;
    pre->next = n;
  }

  if (curr == NULL) {
    // curr = NULL 插入的为尾节点
    obj->tail = n;
  }

  obj->size += 1;
}

/** Delete the index-th node in the linked list, if the index is valid. */
void myLinkedListDeleteAtIndex(MyLinkedList *obj, int index) {
  if (obj == NULL || index < 0 || index >= obj->size) {
    return;
  }

  Node *curr = obj->head;
  Node *pre = NULL;
  while (index-- > 0) {
    pre = curr;
    curr = curr->next;
  }

  Node *n = curr;
  if (pre == NULL) {
    // pre == NULL 删除第一个节点
    obj->head = NULL;
  } else {
    pre->next = curr->next;
  }

  if (curr->next == NULL) {
    // curr->next = NULL 删除的最后一个节点 tail需要重新指向
    // 如果curr为唯一节点 pre则为NULL
    // 此时tail = pre = NULL
    obj->tail = pre;
  }

  free(n);

  obj->size -= 1;
}

void myLinkedListFree(MyLinkedList *obj) {
  if (obj == NULL) {
    return;
  }

  Node *iter = obj->head;
  while (iter) {
    Node *n = iter;
    iter = iter->next;
    free(n);
  }

  free(obj);
}
