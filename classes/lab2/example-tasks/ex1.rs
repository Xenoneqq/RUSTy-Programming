fn main() {
  let mut s = String::from("sample text"); // memory alocation 

  s.push_str(" with some additional text"); 

  println!("{}", s);
}