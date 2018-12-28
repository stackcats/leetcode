# https://leetcode.com/problems/simplify-path/description/

class Solution:
    def simplifyPath(self, path):
        """
        :type path: str
        :rtype: str
        """
        st = []
        for each in path.split('/'):
            if each == '.' or each == '':
                continue
            elif each == '..':
                if len(st) > 0:
                    st.pop()
            else:
                st.append(each)
        return "/" + "/".join(st)
