(define/contract (the-maximum-achievable-x num t)
  (-> exact-integer? exact-integer? exact-integer?)
  (+ num t t))
