fn main() {
  let s1 = String::from("sample");
  let s2 = s1;
  
  println!("{}", s2);
  // println!("{}", s1); // error, the value is invalid
  number_example();
}

fn number_example() {
  let a = 6;
  let b = a; 
  
  println!("{}", a); // this is valid
  println!("{}", b); 
}