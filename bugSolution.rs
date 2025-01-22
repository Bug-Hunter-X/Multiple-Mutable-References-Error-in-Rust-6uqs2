fn main() {
    let mut x = 5;
    {   
        let y = &mut x;
        *y = 10;
    }
    {
        let z = &mut x; 
        *z = 15;
    }

    let a = &mut x; 
    *a = 20; 
} 