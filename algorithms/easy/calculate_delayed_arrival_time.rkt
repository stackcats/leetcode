(define/contract (find-delayed-arrival-time arrivalTime delayedTime)
  (-> exact-integer? exact-integer? exact-integer?)
  (remainder (+ arrivalTime delayedTime) 24))
