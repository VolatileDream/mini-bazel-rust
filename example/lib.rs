use std::io;

pub fn hello() -> io::Result<()> {
  Ok(println!("hello world"))
}
