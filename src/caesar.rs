use std::io::Write;

fn main() {
  let mode = match std::env::args().nth(1) {
    Some(arg) => arg,
    None => {
      std::io::stderr()
        .write(b"Select a mode 'encode' or 'decode'")
        .unwrap();
      return;
    }
  };
  let key = match std::env::args().nth(2) {
    Some(arg) => match arg.parse::<u8>() {
      Ok(v) => v,
      Err(e) => {
        std::io::stderr()
          .write(format!("{}", e).as_bytes())
          .unwrap();
        return;
      }
    },
    None => {
      std::io::stderr()
        .write(b"Please enter a key from 0 to 127")
        .unwrap();
      return;
    }
  };
  let source = match std::env::args().nth(3) {
    Some(arg) => arg,
    None => {
      std::io::stderr()
        .write(b"Please enter the source filename")
        .unwrap();
      return;
    }
  };
  let destination = match std::env::args().nth(4) {
    Some(arg) => arg,
    None => {
      std::io::stderr()
        .write(b"Please enter the destination filename")
        .unwrap();
      return;
    }
  };
  let contents = match std::fs::read_to_string(&source) {
    Ok(s) => s,
    Err(e) => {
      std::io::stderr()
        .write(format!("{}", e).as_bytes())
        .unwrap();
      return;
    }
  };
  let bytes = contents.as_bytes();

  if mode == "encode" {
    let mut res = Vec::<u8>::with_capacity(bytes.len());
    for i in bytes.iter() {
      res.push((*i + key) % 127);
    }
    std::fs::write(destination, res).expect("Destination file not found");
  } else if mode == "decode" {
    let mut res = Vec::<u8>::with_capacity(bytes.len());
    for i in bytes {
      let v = *i as i16 - key as i16;
      let final_v;
      if v < 0 {
        final_v = 127 + v;
      } else {
        final_v = v;
      }
      res.push(final_v as u8);
    }
    std::fs::write(destination, res).expect("Destination file not found");
  } else {
    std::io::stderr()
      .write(b"Invalid mode, use either 'decode' or 'encode'")
      .unwrap();
  }
}
