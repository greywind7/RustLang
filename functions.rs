// In rust statements don't return any value, but expressions do
// The last expression in a function is implicitly returned and alternatively
// The return keyword can also be used
// expressions are not terminated by semi colon
// Otherwise they become statements

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