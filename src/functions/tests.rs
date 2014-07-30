//glob imports are experimental and don't appear to actually work
use functions::compare;
use functions::compare_int;
use functions::compare_pair;
use functions::compare_order;
use functions::contains_zero;
use functions::first_non_zero;
use functions::fib;

// Expression
#[test]
fn compare_true_if_first_greater() {
  assert!(compare(5,4))
}

#[test]
fn compare_false_if_second_greater() {
  assert!(!compare(7,8))
}

// If

#[test]
fn compare_1_if_first_greater() {
  assert!(1 == compare_int(12,8))
}

#[test]
fn compare_0_if_second_greater() {
  assert!(0 == compare_int(3,14))
}

// This passes a 2-tuple. Need to deconstruct.
#[test]
fn compare_pair_true_if_first_greater() {
  assert!(compare_pair((3,2)))
}

#[test]
fn compare_pair_false_if_second_greater() {
  assert!(!compare_pair((1,2)))
}

// Match without binding

#[test]
fn contains_zero_if_first_zero() {
  assert!(contains_zero((0,1)))
}

#[test]
fn contains_zero_if_second_zero() {
  assert!(contains_zero((1,0)))
}

#[test]
fn contains_zero_false_if_no_zero() {
  assert!(!contains_zero((1,2)))
}

// Match with binding

#[test]
fn return_second_if_first_is_zero() {
  assert!(12 == first_non_zero((0,12)))
}

#[test]
fn return_first_if_second_is_zero() {
  assert!(19 == first_non_zero((19,0)))
}

#[test]
fn return_zero_if_both_zero() {
  assert!(0 == first_non_zero((0,0)))
}

#[test]
fn return_zero_if_neither_zero() {
  assert!(0 == first_non_zero((1,1)))
}

// Match with guard
#[test]
fn compare_order_1_if_first_greater() {
  assert!(1 == compare_order((87,1)))
}

#[test]
fn compare_order_0_if_equal() {
  assert!(0 == compare_order((7,7)))
}

#[test]
fn compare_order_minus_1_if_second_greater() {
  assert!(-1 == compare_order((87,111)))
}

// Fibonacci, zero-based
// current Rust does not provide a way to guarantee tail call optimisation

#[test]
fn first_fibonacci_is_zero() {
  assert!(0 == fib(0))
}

#[test]
fn second_fibonacci_is_one() {
  assert!(1 == fib(1))
}

#[test]
fn third_fibonacci_is_one() {
  assert!(1 == fib(2))
}

#[test]
fn fourth_fibonacci_is_two() {
  assert!(2 == fib(3))
}

#[test]
fn fifth_fibonacci_is_three() {
  assert!(3 == fib(4))
}

#[test]
fn sixth_fibonacci_is_five() {
  assert!(5 == fib(5))
}

#[test]
fn tenth_fibonacci_is_thirtyfour() {
  assert!(34 == fib(9))
}
