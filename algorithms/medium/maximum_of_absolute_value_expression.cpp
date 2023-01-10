class Solution {
public:
  int maxAbsValExpr(vector<int> &arr1, vector<int> &arr2) {
    vector<array<int, 4>> mat(arr1.size());
    for (int i = 0; i < arr1.size(); i++) {
      array<int, 4> arr = {arr1[i] + arr2[i] + i, arr1[i] + arr2[i] - i,
                           arr1[i] - arr2[i] + i, arr1[i] - arr2[i] - i};
      mat[i] = move(arr);
    }

    int ans = 0;
    for (int j = 0; j < 4; j++) {
      int ma = INT_MIN, mi = INT_MAX;
      for (int i = 0; i < mat.size(); i++) {
        ma = max(mat[i][j], ma);
        mi = min(mat[i][j], mi);
      }
      ans = max(ans, ma - mi);
    }

    return ans;
  }
};
