// structures on heap in rust can only have one owner at a time
// assigning value to another variable transfers ownership

fn main()
{
    let s1 = String::from("Cyka Blyat");
    let s2 = s1;
    // At this point in the scope,
    // the ownership of s1 has been transferred to s2
    // any attempt to use s1 now will result in error

    println!("s2 = {}", s2);

    // The heap can be made to clone however
    // Now there are two separate copies rather than two references to the same heap
    let s3 = s2.clone();
    println!("s3 = {} s2 = {}", s3,s2);

    // If we pass a string as an argument, then the ownership is transferred as well
    // we can return the ownership by returning the string/tuple
    // But that is tedios so we use the concept of borrowing through references
    let st = String::from("Hi kai");
    borrow(&st);
    println!("Confirming that the string still has its own ownership! It says {}",st);

    // Mutable references are used to be able to change the string
    let mut tw = String::from("Its Timo time");
    mut_borrow(&mut tw);
    println!("TW: {}",tw);

    slice();
}

fn borrow(x:&String)
{
    println!("String says {}", x);
}

fn mut_borrow(mut x:&mut String)
{
    x.push_str(". His finishing gae tho.");
    let mut y = &mut x;
    y.push_str(" Kai bes!");
    // here we have two mutable references to the same variable
    // to prevent race conditions, we cant have two mutable references to the same variable in their scope
    // scope starts from the declaration to the last use
    // Following line if uncommented will generate error
    // println!("{}{}",x,y);
}

fn slice()
{
    // Slicing is basically slicing in any language
    let boo = "This line is gae!";
    // The slice is a &str type and is immutable
    let bar = &boo[..5];
    println!("Bar = {}",bar);
    let foo = &boo[5..];
    println!("Foo = {}",foo);
    // Slicing can also be done on arrays
    // mutating last 5 elements to have +10 value
    let mut x:[i32;10] = [0;10];
    for i in 0..10
    {
        x[i] = i as i32;
    }

    let mut y = &mut x[5..];

    for val in y.iter_mut()
    {
        *val += 10;
    }

    for val in x.iter()
    {
        print!("{} ",val);
    }
    println!();
}