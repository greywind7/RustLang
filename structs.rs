// Structures in Rust are the equivalents of classes
// Rust is not object oriented because it doesnt have 
// object oriented features like inheritence etc

struct Student {
    name: String,
    age: u8,
    pass: bool,
}

fn main()
{
    // This is how a structure is declared
    let stu = Student {
        name: String::from("Charlie"),
        age: 17,
        pass:true
    };
    println!("Name: {}, Age: {}",stu.name,stu.age);

    // We can also use a function as an explicit constructor
    let stu1 = make_stu(String::from("Ivan"), 19, false);
    println!("Name: {}, Age: {}",stu1.name,stu1.age);

    // A struct can also be initialized from an instance of another struct
    let stu2 = Student{
        name:String::from("Sasha"),
        // Take rest of the parameters from stu1
        ..stu1
    };
    println!("Name: {}, Age: {}",stu2.name,stu2.age);

}

fn make_stu(name:String,age:u8,pass:bool) -> Student {
    Student {
        // if the fn parameters are same, then they can be passed directly
        // This is an initialization shorthand
        name,
        age,
        pass
    }
}