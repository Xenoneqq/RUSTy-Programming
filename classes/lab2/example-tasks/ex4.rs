fn main() {
  let s1 = String::from("sample text");
  let s2 = process_text(s1);
  println!("{}", s2);
}

fn process_text(s: String) -> String {
  s.to_uppercase()
}