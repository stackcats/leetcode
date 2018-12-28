// https://leetcode.com/problems/group-anagrams/

import (
	"sort"
	"strings"
)

func key(s string) string {
	t := strings.Split(s, "")
	sort.Strings(t)
	return strings.Join(t, "")
}

func groupAnagrams(strs []string) [][]string {
	dt := map[string][]string{}

	for _, s := range strs {
		k := key(s)
		_, ok := dt[k]
		if !ok {
			dt[k] = []string{s}
		} else {
			dt[k] = append(dt[k], s)
		}
	}

	ans := [][]string{}
	for _, v := range dt {
		ans = append(ans, v)
	}
	return ans
}
