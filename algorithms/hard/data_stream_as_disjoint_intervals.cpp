class SummaryRanges {
public:
  map<int, vector<int>> mp;
  SummaryRanges() {}

  void addNum(int value) {
    auto higher = mp.lower_bound(value);
    auto lower = higher == mp.begin() ? mp.end() : prev(higher);
    if (lower != mp.end() && higher != mp.end() &&
        lower->second[1] + 1 == value && value + 1 == higher->second[0]) {
      lower->second[1] = higher->second[1];
      mp.erase(higher);
    } else if (lower != mp.end() && lower->second[1] + 1 >= value) {
      lower->second[1] = max(lower->second[1], value);
    } else if (higher != mp.end() && value + 1 >= higher->second[0]) {
      higher->second[0] = min(higher->second[0], value);
    } else {
      mp.insert({value, {value, value}});
    }
  }

  vector<vector<int>> getIntervals() {
    vector<vector<int>> arr;
    for (const auto &[k, v] : mp) {
      arr.push_back(v);
    }
    return arr;
  }
};

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * SummaryRanges* obj = new SummaryRanges();
 * obj->addNum(value);
 * vector<vector<int>> param_2 = obj->getIntervals();
 */
