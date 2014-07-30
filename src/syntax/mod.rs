mod tests;

fn compare(a: int, b:int) -> bool {
  a>b
}

fn compare_int(a: int, b: int) -> int {
  if a>b {
    1
  } else {
    0
  }
}

fn compare_pair(pair:(int,int)) -> bool {
  let (a,b) = pair;
  compare(a,b)
}

fn contains_zero(pair:(int,int)) -> bool {
  match pair {
    (0,_) => true,
    (_,0) => true,
    _     => false
  }
}

fn first_non_zero(pair:(int,int)) -> int {
  match pair {
    (0,a) => a,
    (a,0) => a,
    _     => 0
  }
}

fn compare_order(pair:(int,int)) -> int {
  match pair {
    (a,b) if a>b  => 1,
    (a,b) if a<b  => -1,
    (a,b) if a==b => 0,
    _             => 0
  }
}

fn fib(n: int) -> int {
  match n {
    0 => 0,
    1 => 1,
    m => fib(m-1) + fib(m-2)
  }
}

fn div(dividend: int, divisor: int) -> Option<int> {
  if divisor == 0 { None }
  else { Some(dividend / divisor) }
}

fn getOrMinusOne(val: Option<int>) -> int {
  val.unwrap_or(-1)
}

fn double(val: Option<int>) -> Option<int> {
  val.map(|x| x * 2)
}