class Solution {
public:
  int size = 0;
  int snakesAndLadders(vector<vector<int>> &board) {
    size = board.size();
    int n = board.size() * board.size();
    queue<int> q;
    q.push(1);
    unordered_map<int, int> mp;
    mp.insert({1, 0});
    while (!q.empty()) {
      int label = q.front();
      q.pop();
      int step = mp[label];
      if (label == n) {
        return step;
      }
      for (int i = 1; i <= 6; i++) {
        int next = label + i;
        if (next > n)
          break;
        auto ndx = labelToIndex(next);
        if (board[ndx.first][ndx.second] != -1) {
          next = board[ndx.first][ndx.second];
        }
        if (mp.find(next) == mp.end()) {
          q.push(next);
          mp.insert({next, step + 1});
        }
      }
    }
    return -1;
  }
  pair<int, int> labelToIndex(int label) {
    int i = size - (label - 1) / size - 1;
    int j = (label - 1) % size;
    if ((size - i) % 2 == 0) {
      j = size - 1 - j;
    }
    return {i, j};
  }
};
