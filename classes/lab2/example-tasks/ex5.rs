fn main() {
  let s1 = String::from("sample");
  let s2 = process_text(&s1);  // lends the s1 value through a reference
  println!("{} -> {}", s1, s2) // both s1 and s2 are valid
}

fn process_text(s: &String) -> String { // borrow s value and return new string
  s.to_uppercase()
}