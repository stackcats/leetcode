// https://leetcode.com/problems/verifying-an-alien-dictionary/

func isAlienSorted(words []string, order string) bool {
	dt := map[byte]int{}
	for i := range order {
		dt[order[i]] = i
	}

	for i := 0; i < len(words)-1; i++ {
		flag := 0
		for k := 0; k < len(words[i]) && k < len(words[i+1]); k++ {
			if dt[words[i][k]] > dt[words[i+1][k]] {
				flag = 1
				break
			} else if dt[words[i][k]] < dt[words[i+1][k]] {
				flag = -1
				break
			}
		}

		if flag == 1 {
			return false
		}

		if flag == 0 && len(words[i]) > len(words[i+1]) {
			return false
		}

	}

	return true
}
