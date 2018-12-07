// https://leetcode.com/problems/jump-game/description/

#define max(a, b) ((a) < (b) ? (b) : (a))

bool canJump(int *nums, int numsSize) {
  int max_right = 0; // 当前能到达的最远距离

  for (int i = 0; i < numsSize - 1; i++) {
    if (max_right < i) {
      // 无法到达i
      break;
    }
    max_right = max(nums[i] + i, max_right);
  }
  return max_right >= numsSize - 1;
}
