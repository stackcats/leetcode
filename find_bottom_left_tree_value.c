// https://leetcode.com/problems/find-bottom-left-tree-value/description/

typedef struct QNode_ {
  void *val;
  struct QNode_ *next;
} QNode;

QNode *qnode_init(void *val) {
  QNode *n = (QNode *)malloc(sizeof(QNode));
  n->val = val;
  n->next = NULL;
  return n;
}

typedef struct {
  QNode *front;
  QNode *rear;
} Queue;

Queue *queue_init() {
  Queue *q = (Queue *)malloc(sizeof(Queue));
  q->front = NULL;
  q->rear = NULL;
  return q;
}

void queue_free(Queue *q) {
  assert(q);

  while (q->front) {
    QNode *n = q->front;
    q->front = q->front->next;
    free(n);
  }

  free(q);
}

void enqueue(Queue *q, void *val) {
  assert(q);

  QNode *n = qnode_init(val);
  if (q->front == NULL) {
    q->front = n;
  } else {
    q->rear->next = n;
  }

  q->rear = n;
}

void *dequeue(Queue *q) {
  assert(q && q->front);

  QNode *n = q->front;
  q->front = q->front->next;
  void *val = n->val;
  free(n);
  return val;
}

int findBottomLeftValue(struct TreeNode *root) {
  Queue *curr = queue_init();
  Queue *nextLevel = NULL;

  enqueue(curr, root);
  int leftmost;
  struct TreeNode *node;
  while (curr->front != NULL) {
    nextLevel = queue_init();
    int first = 1;
    while (curr->front != NULL) {
      node = dequeue(curr);
      if (first) {
        leftmost = node->val;
        first = 0;
      }
      if (node->left) {
        enqueue(nextLevel, node->left);
      }
      if (node->right) {
        enqueue(nextLevel, node->right);
      }
    }

    Queue *tmp = curr;
    curr = nextLevel;
    free(tmp);
  }

  free(curr);
  return leftmost;
}
