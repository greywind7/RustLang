fn main()
{
    // Option enum has two values Some(T) or None
    // Thi is as close you get to using nullptr in safe rust
    let mut x = Some(2);
    // While unwrap() method consumes the option,
    // take() replaces the Some() with None and gives the Some(x) value 
    // this x can be used, remember that
    let mut y = x.take();
    println!("y {:?} x {:?}",y,x);
    // match allows you to work with enums like a switch statement
    match y.take(){
        Some(a) => {
            println!("a is {}",a);
            x = Some(a);
        }
        None =>{println!("Well fuck");}
    }
    println!("y {:?} x {:?}",y,x);
}