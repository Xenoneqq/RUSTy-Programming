fn main(){
  kolko_krzyzyk();
  println!("is -4 positive? : {}" , is_positive(-3))
}

fn kolko_krzyzyk(){
  let mut board = [['x' ; 3];3];
  println!("{:?}" , board);
}

fn is_positive(n:i32) -> i32{
  if n > 0 {
    return 1
  }
  0
}