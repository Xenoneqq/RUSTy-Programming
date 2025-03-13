fn main() { 
  let x = 5; 
  let x = x + 1; 
  { 
      // zmienna X została utworzona lokalnie w tym wcięciu
      // poza nim wartosc X pozostala = 6
      let x = x * 2; 
      println!("The value of x in the inner scope is: {}", x); 
  } 
  
  println!("The value of x is: {}", x); 
}