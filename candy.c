// https://leetcode.com/problems/candy/description/

#define max(a, b) ((a) < (b) ? (b) : (a))

int candy(int *ratings, int ratingsSize) {
  int *d = calloc(ratingsSize, sizeof(int));
  for (int i = 0; i < ratingsSize; i++) {
    d[i] = 1;
  }
  for (int i = 1; i < ratingsSize; i++) {
    if (ratings[i] > ratings[i - 1]) {
      d[i] = d[i - 1] + 1;
    }
  }

  int ans = d[ratingsSize - 1];
  for (int i = ratingsSize - 2; i >= 0; i--) {
    if (ratings[i] > ratings[i + 1]) {
      d[i] = max(d[i], d[i + 1] + 1);
    }
    ans += d[i];
  }
  free(d);
  return ans;
}
