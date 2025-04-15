fn main(){
  let d1 = 98222; // decimal
  let d2 = 98_222; //  _ is a visual separator
  let h = 0xff; // hex
  let o = 0o77; // octal
  let b = 0b1111_0000; // binary
  let x = b'A'; // byte (only for u8)

  println!("d2 value : {}", d2);
  println!("h value : {}", h);
}