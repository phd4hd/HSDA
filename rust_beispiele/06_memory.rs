use std::mem::drop; // ähnlich zu free()

fn main() {

  let s = "Hallo".to_string();
  drop(s);

  println!("s = {}", s);
}

