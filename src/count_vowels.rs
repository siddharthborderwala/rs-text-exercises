use std::collections::HashMap;

fn main() {
  let input = std::env::args()
    .nth(1)
    .expect("Enter a string to count the number of vowels");
  let mut map = HashMap::<char, usize>::new();
  map.insert('a', 0);
  map.insert('e', 0);
  map.insert('i', 0);
  map.insert('o', 0);
  map.insert('u', 0);
  for i in input.chars() {
    if map.contains_key(&i) {
      // we unwrap because it is certain that vowel key exists in the hashmap
      map.insert(i, *map.get(&i).unwrap() + 1);
    }
  }
  let total = map.into_iter().fold(0, |total, (character, count)| {
    println!("Occurrences of {}: {}", character, count);
    total + count
  });
  println!("Total vowel count: {}", total);
}
