(define/contract (smallest-even-multiple n)
  (-> exact-integer? exact-integer?)
    (lcm 2 n))
