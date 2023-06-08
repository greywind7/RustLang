// In rust statements don't return any value, but expressions do
// The last expression in a function is implicitly returned and alternatively
// The return keyword can also be used
// expressions are not terminated by semi colon
// Otherwise they become statements

// Expressions evaluate to a value
// Function calls, macro calls, a new scope block are all expressions 

// Functions can be declared anywhere as long as they can be seen by the caller
// Effectively you can call functions even if they are defined below the current one

// In function signature, the type of the parameters must be declared

fn sum(x:i32,y:i32)->i32{
    x+y
}

fn main(){
    let (x,y) = (10,5);
    hello();
    println!("{}",sum(x,y));
}

fn hello()
{
    println!("Hello World");
}