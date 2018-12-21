// https://leetcode.com/problems/random-pick-with-blacklist/

// 1000ms 速度非常慢

import (
	"math/rand"
	"time"
)

type Solution struct {
	Black map[int]int
	White []int
	N     int
	Kind  int
}

func Constructor(N int, blacklist []int) Solution {
	sol := Solution{N: N}
	sol.White = make([]int, 0)
	sol.Black = map[int]int{}
	for _, n := range blacklist {
		sol.Black[n] = 1
	}

	if float64(len(blacklist))/float64(N) > 0.7 {
		sol.Kind = 1
		for i := 0; i < N; i++ {
			_, ok := sol.Black[i]
			if !ok {
				sol.White = append(sol.White, i)
			}
		}
	} else {
		sol.Kind = 2
	}

	return sol
}

func (this *Solution) Pick() int {
	rand.Seed(time.Now().UnixNano())
	if this.Kind == 1 && len(this.White) > 0 {
		return this.White[rand.Intn(len(this.White))]
	}

	for {
		n := rand.Intn(this.N)
		_, ok := this.Black[n]
		if !ok {
			return n
		}
	}

	return 0
}

/**
 * Your Solution object will be instantiated and called as such:
 * obj := Constructor(N, blacklist);
 * param_1 := obj.Pick();
 */
