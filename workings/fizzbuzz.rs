fn is_three(num: int) -> bool {
    num % 3 == 0
}

fn is_five(num: int) -> bool {
    num % 5 == 0
}

fn is_fifteen(num: int) -> bool {
    num % 15 == 0
}

#[test]
fn test_is_three_with_not_three() {
    if is_three(1) {
        fail!("One is not three");
    }
}

#[test]
fn test_is_three_with_three() {
  assert!(is_three(3))
}

#[test]
fn test_is_five_with_not_five() {
    if is_five(2) {
        fail!("Two is not five");
    }
}

#[test]
fn test_is_five() {
  assert!(is_five(5))
}

#[test]
fn test_is_fifteen_with_not_fifteen() {
    if is_fifteen(12) {
        fail!("Fifteen should be fifteen");
    }
}

#[test]
fn test_is_fifteen() {
  assert!(is_fifteen(15))
}

fn main() {
    for num in range(1, 101) {
        println(
          if is_fifteen(num) { ~"FizzBuzz" }
          else if is_three(num) { ~"Fizz" }
          else if is_five(num) { ~"Buzz" }
          else { num.to_str() }
        );
    }
}