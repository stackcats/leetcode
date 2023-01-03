class Solution {
public:
    int numOfSubarrays(vector<int>& arr, int k, int threshold) {
        int sum = accumulate(arr.begin(), arr.begin() + k, 0);
        int ct = (sum / k) >= threshold ? 1 : 0;
        for (int i = k; i < arr.size(); ++ i) {
          sum -= arr[i-k];
          sum += arr[i];
          if (sum / k >= threshold) ct++;
        }
        return ct;
    }
};
