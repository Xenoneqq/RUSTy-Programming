struct Person{
  name: String,
  age: u32
}

fn main(){
  let name = "Jan";
  let age = 21;
  let john = Person {name: name.to_string(), age};

  println!("{} , {}", john.name, john.age);
}