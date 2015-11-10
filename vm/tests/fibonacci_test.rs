#![feature(convert)]

mod support;
use support::verify_output;

#[test]
fn fibonacci_test() {
    let program = "
(def fib (lambda (value)
          (cond (equal value 0)
            0
            (cond (equal value 1)
              1
              (add (fib (sub value 1)) (fib (sub value 2)))
            )
          )
         )
)";

    verify_output(format!("{} (dump (fib 0))", program).as_str(), "0_i32");
    verify_output(format!("{} (dump (fib 1))", program).as_str(), "1_i32");
    verify_output(format!("{} (dump (fib 2))", program).as_str(), "1_i32");
    verify_output(format!("{} (dump (fib 3))", program).as_str(), "2_i32");
    verify_output(format!("{} (dump (fib 4))", program).as_str(), "3_i32");
    verify_output(format!("{} (dump (fib 5))", program).as_str(), "5_i32");
    verify_output(format!("{} (dump (fib 9))", program).as_str(), "34_i32");
    verify_output(format!("{} (dump (fib 10))", program).as_str(), "55_i32");
}
