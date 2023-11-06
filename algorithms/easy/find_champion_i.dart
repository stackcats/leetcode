class Solution {
  int findChampion(List<List<int>> grid) {
    int ct = 0;
    int team = 0;
    for (int i = 0; i < grid.length; i++) {
      int len = grid[i].where((e) => e == 1).length;
      if (ct < len) {
        ct = len;
        team = i;
      }
    }
    return team;
  }
}
