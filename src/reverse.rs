// fn main() {
//   let input = match std::env::args().nth(1) {
//     Some(v) => v,
//     None => "".to_string(),
//   };
//   let mut rev = input.into_bytes();
//   rev.reverse();
//   for i in rev {
//     print!("{}", i as char);
//   }
// }

fn main() {
  let input = match std::env::args().nth(1) {
    Some(v) => v,
    None => "".to_string(),
  };
  let mut result: Vec<String> = input.chars().map(|s| s.to_string()).collect();
  result.reverse();
  println!("{}", result.join(""));
}
