fn main()
{
    // Variables in rust are immutable by default, 
    // to make them mutable use keyword mut
    mutability();

    // There are constants as well that must be annotated
    // Always immutable
    // Follow scope rules
    consts();

    // Shadowing
	// Its when a variable shadows another variable of the same name
	// Helps to change variable type and reuse variable names when they are no longer required
	// In effect a new variable is created so its not the same as using mut
	// We can use shadowing to change the type of mutable variables as well
	shadow();
}

fn mutability() {
    let mut x = 10;
    println!("MUTABILITY\nx = {}",x);
    x = 11;
    println!("Changing previous value of x to {x}\n");
}

fn consts() {
    const SECS_IN_DAY: u32 = 24 * 60 * 60; 
    println!("Fun fact, there are {SECS_IN_DAY} seconds in a day. You cant change that :D\n");
}

fn shadow() {
	println!("SHADOWING");
	let x = 99;
	println!("x is i32 with value {x}");
	let x = 99.99;
	{
		println!("This x, from the outer scope, which shadows the previous one to become a floating {x}");
		let x = x * 2 as f32; // Since this is the first time we are making any assumtions about the outside x, its automatically taken to be f32. had it been explicitly set to f64, this line would have given an error
		println!("Inner x is {x}");
	}
	println!("But outside the outer scope x is still {x}");
}