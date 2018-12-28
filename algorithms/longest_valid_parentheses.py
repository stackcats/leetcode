# https://leetcode.com/problems/longest-valid-parentheses/description/

# 使用stack辅助计算
class Solution:
    def longestValidParentheses(self, s):
        """
        :type s: str
        :rtype: int
        """
        st = [-1]
        
        ans = 0
        
        for i in range(len(s)):
            if s[i] == '(':
                st.append(i)
            else:
                st.pop()
                if len(st) > 0:
                    l = i - st[-1] # 当前最大长度
                    ans = max(l, ans)
                else:
                    # stack为空时 把')'的下标push到stack中作为边界
                    st.append(i)
                    
        return ans
    
