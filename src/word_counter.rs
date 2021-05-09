use std::io::Write;

fn main() {
  let filename = match std::env::args().nth(1) {
    Some(name) => name,
    None => {
      std::io::stderr()
        .write(b"Please enter the filename")
        .unwrap();
      return;
    }
  };
  let content = match std::fs::read_to_string(&filename) {
    Ok(v) => v.to_string(),
    Err(e) => {
      std::io::stderr()
        .write(format!("{} - {}", filename, e).as_bytes())
        .unwrap();
      return;
    }
  };
  if content.len() == 0 {
    std::io::stderr().write("Empty file".as_bytes()).unwrap();
    return;
  }
  let mut words: usize = 0;
  for i in content.chars() {
    if i == ' ' {
      words += 1;
    }
  }
  words += 1;
  println!("Number of words: {}", words);
}
