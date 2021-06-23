// if, else and else if statements work as expected minus brackets
// They can also be used to assign values
// Both if and else must return the same type of value

fn main(){
    let x = 10;

    let y = if x < 10 {5} else {15};
    println!("y has the value {}",y);

    // Infinite loop has a keyword loop

    let mut j = 0;

    loop{
        println!("Loop {}",j);
        j+=1;
        if j == 5{break;}
    }

    // While loops work as expected

    let mut i = 5;

    while i > 0 {
        println!("Hi {}",i);
        i -= 1;
    }

    // There is a while let loop too
    // It matches conditions and when None arrives, it aborts
    let mut tee = vec![1,2,3,4];
    while let Some(v) = tee.pop() {
        println!("Value is {}",v);
    }

    // for loops work like in python using the iter method

    let mut nums = [12,34,122,443,123];

    println!("Traversing the array!");
    for val in nums.iter(){
        println!("Iterating ... {}",val);
    }

    // can also be used to change values using a mutable iterator
    println!("\nChanging the array");
    for val in nums.iter_mut(){
        *val += 10;
        println!("Iterating ... {}",i);
    }

    // Using range is also possible
    println!("\nUsing ranges");
    for i in 0..5{
        println!("Element {} is {}", i+1, nums[i]);
    }
}