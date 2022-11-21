class Solution {
public:
    int nearestExit(vector<vector<char>>& maze, vector<int>& entrance) {
      queue<array<int, 3>> q;
      q.push({entrance[0], entrance[1], 0});
      int dirs[4][2] = {{-1, 0}, {0, 1}, {1, 0}, {0, -1}};
      while (!q.empty()) {
        auto [x, y, ct] = q.front(); q.pop();
        if ((x == 0 || x == maze.size() - 1 || y == 0 ||
            y == maze[0].size() - 1) && (x != entrance[0] || y != entrance[1])) {
          return ct;
        }
        for (auto& [dx, dy] : dirs) {
          int nx = x + dx, ny = y + dy;
          if (nx >= 0 && nx < maze.size() && ny >= 0 && ny < maze[0].size() && maze[nx][ny] == '.') {
            q.push({nx, ny, ct + 1});
            maze[nx][ny] = '+';
          }
        }
      }
      return -1;
    }
};
