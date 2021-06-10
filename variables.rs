fn main()
{
    // Variables in rust are immutable by default, 
    // to make them mutable use keyword mut
    let mut x = 10;
    println!("x = {}",x);

    x = 69;
    println!("Ha, x my boi {}, is nice", x);

    // SCALAR DATA TYPES: Single values  
    // Integers, u for unsigned, s for signed
    // By default if not specified i32 is taken
    // Can use 8,16,32,64,128 bits
    let (x1,mut x2) : (i32,i64) = (std::i32::MAX,214000000000+1); 
    // This is how to multideclare variables, type is optional
    // Note: using std::i32 in an i64 variable throws error
    println!("x1 is {}, x2 is {}",x1,x2);

    // Float has two types, one uses 32 bits other 64
    // Default is 64
    let f1 = std::f64::MAX;
    let f2 = std::f32::MIN;
    println!("f1 is {}, f2 is {}",f1,f2);

    // Bool data type
    let boo = 50 < 30;
    println!("Lets find out if its true or false\nboo = {}", boo);

    // Char is not an ascii value, its a unicode value
    let c1 = '\u{1F601}';
    println!("Kinda excited for the output {}",c1);

    // Compound data types: That can have more than one value
    // tuples, can have a single or different data types
    // Element access by dot (.)
    let tutu : (i32, f64, char) = (69, 420.11, 'R');
    println!("Integer is {}, float {} and char {}",tutu.0,tutu.1,tutu.2);
    // Destructuring a tuple
    let (x,y,z) = tutu;
    println!("Now the same thing but with the destructured variables {}{}{}", x,y,z);
    // Arrays have multiple values of same data type
    let days = ["Monday", "Tuesday", "Thursday"];
    println!("Day 2 in the week is {}", days[1]);
    // Arrays in rust dont allow illegal memory access, it directly terminates 
    // Creating fixed size arrays with initialization
    let flots: [f32;3] = [0.5;3];
    println!("Checking for array creation {}", flots[2]);
    let ints2: [[i64;3];4] = [[99;3];4]; // Pay attencacion for the sizes though
    println!("Checking for 2d array creation {}", ints2[3][2]);
}