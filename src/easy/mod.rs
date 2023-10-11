pub mod roman_to_int;

pub fn run() {
  display_roman_to_int("XIV");
  display_roman_to_int("XLII");
}

fn display_roman_to_int(s: &str) {
  println!("{} -> {}", s, roman_to_int::solve(String::from(s)));
}