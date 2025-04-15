// Musi być znana rzy kompilacji (nie może być zmieniana podczas działania)
const DB_PORT : u32 = 5432;

fn main() {
  number_print();
  text_length();
  mutable_variable();
  print_constant();
  float_number();
  tuple_example();
  println!("Value is {}", function(0.2))
}

// Zmienne nie sa modyfikowalne same w sobie
fn number_print(){
  let number : u32 = 5;
  // number = 6; // nie zadziala
  println!("The value is {}",number);
}

// poprawne nadpisanie wartosci zmiennych
fn text_length(){
  let word = "hello";
  let word = word.len();
  println!("The length of hello is {}",word);
}

fn mutable_variable(){
  let mut number : u32 = 6;
  println!("The number is {}" , number);
  number = 1;
  println!("The number now is {}" , number);
}

fn print_constant(){
  println!("The port is : {}" , DB_PORT);
}

fn float_number(){
  let f : f32 = 4.25;
  println!("This is a float value {}" , f);
}

fn tuple_example(){
  let user = ("Cool Dude" , 32, 23456.75);
  let (user_name , level, gold) = user;
  println!("User: {} / Level: {} / Gold: {}" , user.0, user.1, user.2);
  println!("Shorter : {:?}" , user);
  println!("Shorter again : {}, {}, {}",user_name, level, gold);
}

fn function(num:f32) -> f32{
  num * 2.0
}
