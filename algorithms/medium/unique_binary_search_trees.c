// https://leetcode.com/problems/unique-binary-search-trees/description/

// https://en.wikipedia.org/wiki/Catalan_number
int numTrees(int n) {
  return n == 1 ? 1 : (long long)numTrees(n - 1) * (4 * n - 2) / (n + 1);
}
