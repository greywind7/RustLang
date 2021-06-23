// Closures are annoynymous functions
// These are functions within functions and stores in a variable

fn main(){
    let mut x = 10;
    println!("X is {}",x);
    // Closures have access to all the variables in the scope declared before them
    let mut inc_x = ||{
        x+=5;
    };
    inc_x();
    println!("X is now {}\nIs sum of 2,3 even? {}",x,sum_chk(2, 3));
}

fn sum_chk(a:i32,b:i32)->bool{
    // parameters in closures are passed withibn || 
    // Parameter type and return type is inferred automatically
    let chk = |x|{
        x % 2 == 0
    };
    return chk(a+b);
}