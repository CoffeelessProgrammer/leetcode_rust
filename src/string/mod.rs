pub mod p0013_roman_to_int;
pub mod p0067_add_binary;

pub fn run() {
  display_roman_to_int("XIV");
  display_roman_to_int("XLII");
}

fn display_roman_to_int(s: &str) {
  println!("{} -> {}", s, p0013_roman_to_int::solve(s));
}