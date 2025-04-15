fn main(){
  simple_loop();
  simple_while();
  simple_for();
}

fn simple_loop(){
  println!("loop");
  let mut i = 3;
  loop {
    println!("Wartosc i = {}" , i);
    i -= 1;
    if i <= 0 {break;}
  }
}

fn simple_while(){
  println!("while");
  let mut i = 3;
  while i > 0 {
    println!("Wartosc i = {}", i);
    i -= 1;
  }
}

fn simple_for(){
  println!("for");
  let num = [1,2,3,4,5];
  for i in num {
    println!("Wartosc w tablicy to : {}" , i);
  }
}