fn main() {
    // Rust is a statically typed language so it must know the data types at compile time
    // SCALAR DATA TYPES: Single values 
    
    // Integers, u for unsigned, s for signed
    // By default if not specified i32 is taken
    // Can use 8,16,32,64,128 bits
    // Architechture based isize and usize equivalent to register bits
    // can use suffixes to denote type as well eg 69u8 if it can be multiple types
    // can break up numbers for better readability eg 69_420 is 69420

    // Integer literals in rust
    // Decimal          No prefix
    // Hex              0x
    // Octal            0o
    // Binary           0b
    // Byte(only u8)    b'<character>'
    ints();

    // Float has two types, one uses 32 bits other 64
    // Default is 64
    floats();

    // Bool data type
    // true and false
    bools();

    // Char is not an ascii value, its a unicode value
    // Must be in single quote
    chars();

    // COMPOUND DATA TYPES: That Can have more than one value

    // Tuples, a collection of values that can have a single or different data types
    // Element access by dot (.)
    tuples();

    // Arrays have multiple values of same data type
    // Array exists on the stack
    arrays();
}

fn ints() {
    let (x1,x2) : (i32,_) = (std::i32::MAX,std::i32::MAX as u64 +1); 
    let (y1,y2) = (69u8,44_000_000i64);
    // This is how to multideclare variables, type is optional
    // Note: using std::i32 in an u64 variable throws error, so we cast it before using

    println!("INTEGERS:\nx1 is {x1}, x2 is {x2}\ny1 is {y1}, y2 is {y2}\n");
}

fn floats() {
    let f1 = std::f64::MAX;
    let f2 = std::f32::MIN;
    println!("FLOATS:\nMax f64, f1 is {}\nMin f32, f2 is {}\n",f1,f2);
}

fn bools() {
    let boo = 5 < 10;
    println!("BOOLEAN:\nLets find out if 5 < 10 is true or false\nboo = {}\n", boo);
}

fn chars() {
    let c1 = '\u{1F601}';
    println!("CHAR:\nKinda excited for the output {}\n",c1);
}

fn tuples() {
    let tup : (i32, f64, char) = (69, 420.11, 'R');
    println!("TUPLES
Tuple of type (i32, f64, char)
Integer is {}, float {} and char {}",tup.0,tup.1,tup.2);
    // Destructuring a tuple
    let (x,y,z) = tup;
    println!("Destructured tuple with variables x,y,z {} {} {}\n", x,y,z);
}

fn arrays() {
    let days = ["Monday", "Tuesday", "Thursday"];
    println!("ARRAYS\nDay 2 in the week is {}", days[1]);
    // Arrays in rust dont allow illegal memory access, it directly terminates 
    // Creating fixed size arrays with initialization
    let flots: [f32;3] = [0.5;3];
    println!("Checking for array creation {}", flots[2]);
    let ints2: [[i64;3];4] = [[99;3];4]; // Pay attention for the sizes though
    println!("Checking for 2d array creation {}", ints2[3][2]);
}