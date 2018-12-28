// https://leetcode.com/problems/trapping-rain-water/description/

#define max(a, b) ((a) < (b) ? (b) : (a))

int trap(int *height, int heightSize) {
  int i = 0;
  int j = heightSize - 1;
  int leftMax = 0;
  int rightMax = 0;
  int ans = 0;

  while (i < j) {
    if (height[i] < height[j]) {
      leftMax = max(leftMax, height[i]);
      ans += leftMax - height[i];
      i++;
    } else {
      rightMax = max(rightMax, height[j]);
      ans += rightMax - height[j];
      j--;
    }
  }

  return ans;
}
