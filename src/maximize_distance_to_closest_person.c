// https://leetcode.com/problems/maximize-distance-to-closest-person/description/

inline int max(int a, int b) { return a < b ? b : a; }

int maxDistToClosest(int *seats, int seatsSize) {
  int ans = -1;
  int d = 0;
  for (int i = 0; i < seatsSize; i++) {
    if (seats[i]) {
      if (ans == -1) {
        // 坐在最左侧
        ans = max(ans, d);
      } else {
        // 坐在中间 最大距离为空座的一半
        ans = max(ans, d / 2);
      }
      d = 1; // seats[i+1]与seats[i]挨着 距离为1
    } else {
      d++;
    }
  }

  return max(ans, d - 1); // 坐在最右侧
}
