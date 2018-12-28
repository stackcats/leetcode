// https://leetcode.com/problems/word-pattern/description/

typedef struct ListNode_ {
  void *data;
  struct ListNode_ *next;
} ListNode;

typedef struct {
  ListNode *head;
  void (*destroy)(void *data);
} List;

List *list_init(void (*destroy)(void *data)) {
  List *lst = malloc(sizeof(List));
  lst->head = NULL;
  lst->destroy = destroy;
  return lst;
}

void list_free(List *lst) {
  while (lst->head) {
    ListNode *n = lst->head;
    lst->head = lst->head->next;
    lst->destroy(n);
  }

  free(lst);
}

void list_ins_head(List *lst, void *data) {
  ListNode *node = malloc(sizeof(ListNode));
  node->data = data;
  node->next = lst->head;
  lst->head = node;
}

typedef struct {
  char *key;
  char val;
} HashNode;

void hashnode_free(void *data) {
  HashNode *node = data;
  free(node->key);
  free(node);
}

typedef struct {
  int buckets;
  List **table;
  unsigned int (*hf)(char *str);
} Hash;

unsigned int BKDRHash(char *str) {
  unsigned int seed = 1313; // 31 131 1313 13131 131313 etc..
  unsigned int hash = 0;

  while (*str) {
    hash = hash * seed + (*str++);
  }

  return (hash & 0x7FFFFFFF);
}

Hash *hash_init(int buckets, unsigned int (*hf)(char *str)) {
  Hash *h = malloc(sizeof(Hash));
  h->buckets = buckets;
  h->table = calloc(buckets, sizeof(List *));
  for (int i = 0; i < buckets; i++) {
    h->table[i] = list_init(hashnode_free);
  }
  h->hf = hf;
  return h;
}

void hash_free(Hash *h) {
  for (int i = 0; i < h->buckets; i++) {
    list_free(h->table[i]);
  }
  free(h->table);
  free(h);
}

int hash_get(Hash *h, char *key, char *val) {
  int bucket = h->hf(key) % h->buckets;
  ListNode *iter = h->table[bucket]->head;
  while (iter) {
    HashNode *node = iter->data;
    iter = iter->next;
    if (!strcmp(key, node->key)) {
      *val = node->val;
      return 1;
    }
  }

  return 0;
}

void hash_set(Hash *h, char *key, char val) {
  int bucket = h->hf(key) % h->buckets;
  ListNode *iter = h->table[bucket]->head;
  while (iter) {
    HashNode *node = iter->data;
    iter = iter->next;
    if (!strcmp(key, node->key)) {
      node->val = val;
      return;
    }
  }

  HashNode *node = malloc(sizeof(HashNode));
  int klen = strlen(key) + 1;
  node->key = calloc(klen, sizeof(char));
  memset(node->key, 0, klen);
  strcpy(node->key, key);
  node->val = val;

  List *lst = h->table[bucket];
  list_ins_head(lst, node);
}

bool wordPattern(char *pattern, char *str) {
  Hash *h = hash_init(1337, BKDRHash);
  char *ph[256];
  memset(ph, 0, sizeof(ph));

  char *token = strtok(str, " ");
  for (int i = 0; pattern[i] != '\0'; i++) {
    if (!token) {
      hash_free(h);
      return 0;
    }

    char val;
    int exist = hash_get(h, token, &val);
    if (exist && ph[pattern[i]]) {
      if (val != pattern[i] || strcmp(token, ph[pattern[i]]) != 0) {
        hash_free(h);
        return 0;
      }
    } else if (!exist && !ph[pattern[i]]) {
      hash_set(h, token, pattern[i]);
      ph[pattern[i]] = token;

    } else {
      hash_free(h);
      return 0;
    }

    token = strtok(NULL, " ");
  }

  hash_free(h);
  return token == NULL;
}
