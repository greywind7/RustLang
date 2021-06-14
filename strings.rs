// strings are of two types, literals and strings on heaps
// literals have fixed size at compile time and have constant reference type &str
// String type is stored on a heap and is mutable, literals are immutable

fn main()
{
    // This is a string literal, fixed size at compile time
    // It is immutable
    let s1 = "strng litera";

    // This is a string 
    let mut s2 = String::from("Cyka");
    println!("s2 now {}",s2);
    s2.push_str(", Blyat");
    println!("s2 now {}",s2);

    // usize is the highest unsigned address that a memory address can have 
    // on 32 bit system it is u32 on 64 it is u64 
    let sz:usize = s2.len();
    println!("s2 has {} length",sz);
}