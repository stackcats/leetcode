class Solution {
public:
    int differenceOfSum(vector<int>& nums) {
        int element_sum = 0;
        int digit_sum = 0;
        for (auto n : nums) {
            element_sum += n;
            while (n > 0) {
                digit_sum += n % 10;
                n /= 10;
            }
        }
        return abs(element_sum - digit_sum);
    }
};
