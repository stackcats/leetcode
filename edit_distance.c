// https://leetcode.com/problems/edit-distance/description/

// 初始状态
// D[i][0] = i
// D[0][j] = j

// 状态转移方程
// a = D[i-1][j] + 1
// b = D[i][j-1] + 1
// c = D[i-1][j-1] (word1[i] == word2[j])
// c = D[i-1][j-1] + 1 (word1[i] != word2[j])
// D[i][j] = min(a, b, c)

#define min(a, b) ((a) < (b) ? (a) : (b))

int minDistance(char *word1, char *word2) {
  int len1 = strlen(word1) + 1;
  int len2 = strlen(word2) + 1;

  int **arr = (int **)malloc(sizeof(int *) * len1);
  for (int i = 0; i < len1; i++) {
    arr[i] = (int *)malloc(sizeof(int) * len2);
    memset(arr[i], 0, sizeof(int) * len2);
  }

  for (int i = 1; i < len1; i++) {
    arr[i][0] = i;
  }

  for (int j = 1; j < len2; j++) {
    arr[0][j] = j;
  }

  for (int i = 1; i < len1; i++) {
    for (int j = 1; j < len2; j++) {
      int a = arr[i - 1][j] + 1;
      int b = arr[i][j - 1] + 1;
      int c = word1[i - 1] == word2[j - 1] ? arr[i - 1][j - 1]
                                           : arr[i - 1][j - 1] + 1;
      arr[i][j] = min(min(a, b), c);
    }
  }

  int res = arr[len1 - 1][len2 - 1];
  for (int i = 0; i < len1; i++) {
    free(arr[i]);
  }

  free(arr);
  return res;
}
