mod m1 {
    pub fn m1_function() {
        println!("Hello from m1_function!");
        m2::m2_function();
    }
    
    pub mod m2 {
        pub fn m2_function() {
            println!("Hello from m2_function!");
            crate::m0_function();
        }
    }
}
 
fn m0_function() {
    println!("Hello from m0_function!");
}
 
fn main() {
    m1::m1_function();
    m1::m2::m2_function();
}