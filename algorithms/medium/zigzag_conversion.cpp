class Solution {
public:
  string convert(string s, int numRows) {
    if (s.size() == 1 || s.size() <= numRows) {
      return s;
    }

    vector<string> arr(s.size());
    int i = 0;
    int step = 1;
    for (auto c : s) {
      arr[i] += c;
      if (i == 0) {
        step = 1;
      } else if (i == numRows - 1) {
        step = -1;
      }
      i += step;
    }
    stringstream res;
    copy(arr.begin(), arr.end(), ostream_iterator<string>(res, ""));
    return res.str();
  }
};
