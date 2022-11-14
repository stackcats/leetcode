(define/contract (convert-temperature celsius)
  (-> flonum? (listof flonum?))
   (list (+ celsius 273.15) (+ 32 (* celsius 1.8))))
