fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; //This line is fine
    let z = y; 
    *z = 15; // This is also fine

    let a = &mut x; 
    *a = 20; // This will cause a compile-time error
}