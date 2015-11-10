(def fib (lambda (value)
          (cond (equal value 0)
            0
            (cond (equal 1 value)
              1
              (add (fib (sub value 1)) (fib (sub value 2)))
            )
          )
         )
)

(dump (fib 0))
