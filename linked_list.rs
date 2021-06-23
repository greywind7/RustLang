// Simple linked list, to be improved upon later
struct tmp{
    a:i32,
    b:i32,
    next:Option<Box<tmp>>
}

impl tmp{
    fn new(a:i32,b:i32) -> Self{
        tmp{
            a,
            b,
            next:None
        }
    }
}

fn main(){
    let mut x = Box::new(tmp::new(2,2));
    // x.a = 5;
    x.next = Some(Box::new(tmp::new(9,4)));
    let y = &mut x.next.unwrap();
    // y.b = 5;
    println!("{} {}",x.a, y.b);
    let z = tmp::new(32, 64);
    println!("{} {}",z.a,z.b)
}