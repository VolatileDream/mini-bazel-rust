import::import! {
  // Rust bindings struggle with importing targets from the root of the repo.
  // So this doesn't work:
  // "//:lib-hello-world" as lhw;
  "//example:lib-hello-world" as lhw;
}

use std::io;

fn main() -> io::Result<()> {
    lhw::hello()
}

