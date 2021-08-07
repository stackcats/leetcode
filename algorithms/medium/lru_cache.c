// https://leetcode.com/problems/lru-cache/description/

typedef struct ListNode_ {
  void *data;
  struct ListNode_ *prev;
  struct ListNode_ *next;
} ListNode;

ListNode *listnode_init(void *data) {
  ListNode *node = malloc(sizeof(ListNode));
  node->data = data;
  return node;
}

typedef struct {
  ListNode *head;
  ListNode *tail;
} List;

List *list_init() {
  List *lst = malloc(sizeof(List));
  lst->head = NULL;
  lst->tail = NULL;
  return lst;
}

void list_free(List *lst) {
  while (lst->head != NULL) {
    ListNode *n = lst->head;
    lst->head = lst->head->next;
    free(n);
  }

  free(lst);
}

void list_ins_head(List *lst, void *data) {
  ListNode *node = listnode_init(data);
  node->next = lst->head;
  if (lst->head) {
    lst->head->prev = node;
  } else {
    lst->tail = node;
  }
  lst->head = node;
}

void *list_remove(List *lst, ListNode *node) {
  if (node == lst->head) {
    lst->head = node->next;
    if (lst->head == NULL) {
      lst->tail = NULL;
    } else {
      node->next->prev = NULL;
    }
  } else {
    node->prev->next = node->next;
    if (node->next == NULL) {
      lst->tail = node->prev;
    } else {
      node->next->prev = node->prev;
    }
  }

  void *data = node->data;
  free(node);
  return data;
}

typedef struct {
  int key;
  int data;
  ListNode *p; // 优先级列表中的节点
} HashNode;

HashNode *hashnode_init(int key, int data) {
  HashNode *node = malloc(sizeof(HashNode));
  node->key = key;
  node->data = data;
  node->p = NULL;
  return node;
}

typedef struct {
  int buckets;
  List **table;
} Hash;

Hash *hash_init(int buckets) {
  Hash *h = malloc(sizeof(Hash));
  h->buckets = buckets;
  h->table = calloc(buckets, sizeof(List *));
  for (int i = 0; i < buckets; i++) {
    h->table[i] = list_init();
  }
  return h;
}

void hash_free(Hash *h) {
  for (int i = 0; i < h->buckets; i++) {
    list_free(h->table[i]);
  }

  free(h->table);
  free(h);
}

unsigned int hash(unsigned int x) {
  x = ((x >> 16) ^ x) * 0x45d9f3b;
  x = ((x >> 16) ^ x) * 0x45d9f3b;
  x = (x >> 16) ^ x;
  return x;
}

HashNode *hash_set(Hash *h, int key, int data) {
  int bucket = hash(key) % h->buckets;
  List *lst = h->table[bucket];
  ListNode *iter = lst->head;
  while (iter != NULL) {
    HashNode *node = iter->data;
    if (key == node->key) {
      // 已存在 更新data
      node->data = data;
      return node;
    }
    iter = iter->next;
  }

  // 不存在 插入
  HashNode *node = hashnode_init(key, data);
  list_ins_head(lst, node);
  return node;
}

HashNode *hash_get(Hash *h, int key) {
  int bucket = hash(key) % h->buckets;
  List *lst = h->table[bucket];
  ListNode *iter = lst->head;
  while (iter != NULL) {
    HashNode *node = iter->data;
    if (key == node->key) {
      return node;
    }
    iter = iter->next;
  }

  return NULL;
}

void hash_remove(Hash *h, int key) {
  int bucket = hash(key) % h->buckets;
  List *lst = h->table[bucket];
  ListNode *iter = lst->head;
  while (iter != NULL) {
    HashNode *node = iter->data;
    if (key == node->key) {
      list_remove(lst, iter);
      free(node);
      return;
    }
    iter = iter->next;
  }
}

typedef struct {
  Hash *hash;
  List *priority; // 双链表维护数据的优先级 head到tail优先级依次降低
  int capacity;
} LRUCache;

LRUCache *lRUCacheCreate(int capacity) {
  LRUCache *cache = malloc(sizeof(LRUCache));
  cache->capacity = capacity;
  cache->hash = hash_init(1337);
  cache->priority = list_init();
  return cache;
}

int lRUCacheGet(LRUCache *obj, int key) {
  HashNode *node = hash_get(obj->hash, key);
  if (node == NULL) {
    return -1;
  }

  // 修改优先级为最高
  int *k = list_remove(obj->priority, node->p);
  list_ins_head(obj->priority, k);

  return node->data;
}

void lRUCachePut(LRUCache *obj, int key, int value) {
  HashNode *node = hash_get(obj->hash, key);
  if (node) {
    // 已经存在直接更新val和优先级
    hash_set(obj->hash, key, value);
    int *key_ = list_remove(obj->priority, node->p);
    list_ins_head(obj->priority, key_);
    return;
  }

  if (obj->capacity <= 0) {
    // 容量满了删除优先级最低的
    ListNode *last = obj->priority->tail;
    int *k = list_remove(obj->priority, last);
    hash_remove(obj->hash, *k);
    free(k); // lRUCachePut函数申请的空间由lRUCachePut函数自己释放
    obj->capacity += 1;
  }

  // 插入hash表并且优先级最高
  node = hash_set(obj->hash, key, value);
  int *data = malloc(sizeof(int));
  *data = key;
  list_ins_head(obj->priority, data);
  node->p = obj->priority->head;
  obj->capacity -= 1;
}

void lRUCacheFree(LRUCache *obj) {
  hash_free(obj->hash);
  list_free(obj->priority);
  free(obj);
}
