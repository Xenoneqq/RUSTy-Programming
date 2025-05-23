fn main() {
  let data = "Rust is great!".to_string();

  get_char(data.clone());

  string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
  data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
  data = data.to_uppercase();

  println!("{}", data);
}