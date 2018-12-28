// https://leetcode.com/problems/maximum-width-of-binary-tree/description/

typedef struct ListNode_ {
  void *data;
  struct ListNode_ *prev;
  struct ListNode_ *next;
} ListNode;

typedef struct {
  ListNode *front;
  ListNode *rear;
  int size;
} Queue;

Queue *queue_init() {
  Queue *q = malloc(sizeof(Queue));
  q->front = q->rear = NULL;
  q->size = 0;
  return q;
}

void queue_free(Queue *q) {
  while (q->front) {
    ListNode *node = q->front;
    q->front = node->next;
    free(node);
  }
  free(q);
}

void enqueue(Queue *q, void *data) {
  ListNode *node = malloc(sizeof(ListNode));
  node->data = data;
  node->prev = NULL;
  node->next = NULL;
  if (q->front == NULL) {
    q->front = node;
  } else {
    q->rear->next = node;
    node->prev = q->rear;
  }
  q->rear = node;
  q->size += 1;
}

void *queue_rm_front(Queue *q) {
  ListNode *node = q->front;
  q->front = node->next;
  if (q->front == NULL) {
    q->rear = NULL;
  } else {
    q->front->prev = NULL;
  }
  q->size -= 1;

  void *data = node->data;
  free(node);
  return data;
}

void *queue_rm_rear(Queue *q) {
  ListNode *node = q->rear;
  q->rear = node->prev;
  if (q->rear == NULL) {
    q->front = NULL;
  } else {
    q->rear->next = NULL;
  }
  q->size -= 1;
  void *data = node->data;
  free(node);
  return data;
}

#define queue_is_empty(q) ((q)->front == NULL)
#define queue_size(q) ((q)->size)
#define queue_front(q) ((q)->front->data)
#define queue_rear(q) ((q)->rear->data)
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int widthOfBinaryTree(struct TreeNode *root) {
  if (!root) {
    return 0;
  }

  Queue *q1 = queue_init();
  Queue *q2 = queue_init();
  enqueue(q1, root);
  int maxWidth = 0;
  while (!queue_is_empty(q1)) {
    if (maxWidth < queue_size(q1)) {
      maxWidth = queue_size(q1);
    }

    while (!queue_is_empty(q1)) {
      struct TreeNode *node = queue_rm_front(q1);
      if (node) {

        enqueue(q2, node->left);

        enqueue(q2, node->right);
      } else {
        enqueue(q2, NULL);
        enqueue(q2, NULL);
      }
    }

    while (!queue_is_empty(q2) && queue_rear(q2) == NULL) {
      queue_rm_rear(q2);
    }

    while (!queue_is_empty(q2) && queue_front(q2) == NULL) {
      queue_rm_front(q2);
    }

    Queue *t = q1;
    q1 = q2;
    q2 = t;
  }

  free(q1);
  free(q2);

  return maxWidth;
}
