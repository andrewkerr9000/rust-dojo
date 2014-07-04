use functions::compare;
use functions::compare_int;
use functions::compare_pair;
use functions::compare_order;
use functions::contains_zero;

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
  assert!(1 == !compare_int(12,8))
}

#[test]
fn compare_0_if_second_greater() {
  assert!(0 == !compare_int(3,14))
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

// Match

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