// https://leetcode.com/problems/implement-stack-using-queues/

typedef struct Node_ {
  int val;
  struct Node_ *next;
} Node;

typedef struct {
  Node *front;
  Node *rear;
  int size;
} Queue;

Queue *queue_init() {
  Queue *q = malloc(sizeof(Queue));
  q->front = NULL;
  q->rear = NULL;
  q->size = 0;
  return q;
}

void queue_free(Queue *q) {
  while (q->front) {
    Node *node = q->front;
    q->front = q->front->next;
    free(node);
  }
  free(q);
}

void enqueue(Queue *q, int val) {
  Node *node = malloc(sizeof(Node));
  node->val = val;
  node->next = NULL;
  if (q->front) {
    q->rear->next = node;
  } else {
    q->front = node;
  }
  q->rear = node;
  q->size += 1;
}

int dequeue(Queue *q) {
  Node *node = q->front;
  q->front = q->front->next;
  if (!q->front) {
    q->rear = NULL;
  }

  q->size -= 1;

  int val = node->val;
  free(node);
  return val;
}

typedef struct {
  Queue *t;
  Queue *q;
} MyStack;

/** Initialize your data structure here. */
MyStack *myStackCreate(int maxSize) {
  MyStack *st = malloc(sizeof(MyStack));
  st->t = queue_init();
  st->q = queue_init();
  return st;
}

/** Push element x onto stack. */
void myStackPush(MyStack *obj, int x) { enqueue(obj->q, x); }

/** Removes the element on top of the stack and returns that element. */
int myStackPop(MyStack *obj) {
  while (obj->q->size > 1) {
    enqueue(obj->t, dequeue(obj->q));
  }

  int val = dequeue(obj->q);

  Queue *x = obj->q;
  obj->q = obj->t;
  obj->t = x;

  return val;
}

/** Get the top element. */
int myStackTop(MyStack *obj) { return obj->q->rear->val; }

/** Returns whether the stack is empty. */
bool myStackEmpty(MyStack *obj) { return obj->q->front == NULL; }

void myStackFree(MyStack *obj) {
  queue_free(obj->q);
  queue_free(obj->t);
  free(obj);
}

/**
 * Your MyStack struct will be instantiated and called as such:
 * struct MyStack* obj = myStackCreate(maxSize);
 * myStackPush(obj, x);
 * int param_2 = myStackPop(obj);
 * int param_3 = myStackTop(obj);
 * bool param_4 = myStackEmpty(obj);
 * myStackFree(obj);
 */
