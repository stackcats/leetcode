class Solution {
public:
    int averageValue(vector<int>& nums) {
      vector<int> evens;
      copy_if(nums.begin(), nums.end(), back_inserter(evens),
              [](int x) { return x % 6 == 0; });
      return evens.size() == 0
                 ? 0
                 : accumulate(evens.begin(), evens.end(), 0) / evens.size();
    }
};
