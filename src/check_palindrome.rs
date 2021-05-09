fn main() {
  let input = std::env::args().nth(1).expect("Need a string to check 😋");
  let buf = input.as_bytes();
  let size = buf.len();

  if size % 2 == 1 {
    let mid = (size - 1) / 2;
    for i in 1..mid {
      if buf[mid - i] != buf[mid + i] {
        println!("{} is not a palindrome", input);
        return;
      }
    }
    println!("{} is a palindrome", input)
  } else {
    let mid = size / 2;
    for i in 1..mid {
      if buf[mid - i - 1] != buf[mid + i] {
        println!("{} is not a palindrome", input);
        return;
      }
    }
    println!("{} is a palindrome", input)
  }
}
