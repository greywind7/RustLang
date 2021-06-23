fn main(){
    // Iterators are used to iterate over some collection
    // The methods .iter() and .iter_mut() return a reference 
    // The method .into_iter takes ownership

    let mut x = [1,2,3,4,5];

    for val in x.iter(){
        println!("Value is {}", val);
    }
    println!();

    // You need * to dereference value because iterators return &T
    for val in x.iter_mut(){
        *val+=7;
        println!("NEW Value is {}", val);
    }
    println!("The array right now is {:?}\n",x);

    let y = vec![1,2,3,4,5];
    // .into_iter() will consume the vector. It is not implemented for an array
    // The value is taken and put in val which is why it must be made mutable
    for mut val in y.into_iter()
    {
        val*=4;
        println!("EVEN NEWer Value is {}", val);
    }
    // For an array into iter is same as iter but for vector it is consumed
    // The foll line if uncommented will generate and error because value borrowed after move
    // println!("The array right now is {:?}\n",y);
    println!();
    // There is another way to traverse an iterator, using the next method
    // It returns the option enum and must be unwrapped
    let mut it = x.iter();
    // it must be made mutable
    for _ in 0..x.len(){
        let tmp = it.next().unwrap();
        println!("Array rn is {}",tmp);
    }
}