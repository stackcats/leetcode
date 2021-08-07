class Solution:
    def fourSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[List[int]]
        """
        nums.sort()
        ans = []
        length = len(nums)
        for i in range(length-3):
            if target > 0 and nums[i] > target:
                return ans
            if i > 0 and nums[i] == nums[i-1]:
                continue
            for j in range(i+1, length-2):
                if j > i + 1 and nums[j] == nums[j-1]:
                    continue
                l = j + 1
                r = length -1                
                while l < r:
                  
                    s = nums[i] + nums[j] + nums[l] + nums[r]
                    if s == target:
                        ans.append([nums[i], nums[j], nums[l], nums[r]])
                        k = 0
                        while k + l < r and nums[k+l] == nums[l]:
                            k += 1
                        l += k
                        r -= 1
                    elif s > target:
                        r -= 1
                    else:
                        l += 1
                
        return ans
