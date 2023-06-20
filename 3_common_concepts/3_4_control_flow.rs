/*
if, else and else if statements work as expected minus brackets
The conditions must be bool
Automatic type casting is not present for bool
*/

/*
They can also be used to assign values because if / else are expressions
Both if and else must return the same type of value
*/

fn main() {
    let x = 10;

    let y = if x < 10 { 5 } else { 15 };
    println!("y has the value {y} because {x} < 10 is {}\n", (x < 10));

    // Infinite loop has a keyword loop

    let mut j = 0;

	// You can also return values from the break statements
    let ret = loop {
        println!("Infinite loop iteration {}", j);
        j += 1;
        if j == 5 {
            break j;
        }
    };
	println!("Loop broke because j became {ret}\n");

    // While loops work as expected

    let mut i = 5;

    while i > 0 {
        println!("Hi {}", i);
        i -= 1;
    }
	println!("");

	// There are loop labels to specify which loop to break out of
	// They start with a single quote

	let mut idx = 1;
	let msg = 'outer: loop{
		println!("Outer loop started");
		'inner: loop {
			println!("Inner loop was called {idx}");
			idx += 1;
			if idx == 5 {
				break 'outer "Loop shall break, enough is enough\n";
			}
			break 'inner;
		}
	};
	println!("{msg}");

    // for loops can traverse the collection directly or through iterators

    let mut nums = [12, 34, 122, 443, 123];

    println!("Traversing the array!");
    for val in nums.iter() {
        println!("Iterating ... {}", val);
    }

    // can also be used to change values using a mutable iterator
    println!("\nChanging the array");
    for val in nums.iter_mut() {
        *val += 10;
        println!("Iterating ...");
    }

    // Using range is also possible
    println!("\nUsing ranges");
    for i in 0..5 {
        println!("Element {} is {}", i + 1, nums[i]);
    }
}
