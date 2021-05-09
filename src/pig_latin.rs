fn main() {
  let vowels = vec!['a', 'e', 'i', 'o', 'u'];
  let input = std::env::args()
    .nth(1)
    .expect("Please enter a word to convert");
  let first_char = input.as_bytes()[0] as char;
  // if first character is a vowel
  if vowels.contains(&first_char) {
    println!("{}ay", input);
  } else {
    let rest = String::from_utf8_lossy(&input.as_bytes()[1..]);
    println!("{}{}ay", rest, first_char);
  }
}
