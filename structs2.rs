// Tuple structures exist in rust
// Special types of structures where parameters are not named

struct RectangleHW(u32,u32);

// Elements are accessed using indices
fn get_tuprec_area(rec:RectangleHW) -> u32{
    rec.0 * rec.1
}

fn main(){
    let rec = RectangleHW(10,20);
    println!("Area is {}", get_tuprec_area(rec));

    let rec2 = make_rec(30, 69);
    rec2.givDetes("I am a friggin rectangle :')");
    println!("Area of this rectangle is {}",rec2.area());

    let rec3 = Rectangle::new(99, 82);
    println!("This is a new rectangle made with new().
Height {} Width {} Area {}",rec3.height,rec3.width,rec3.area());
}

// Struct methods can be implemented
struct Rectangle {
    height:u32,
    width:u32
}


// This is one way of making this constructor, otherwise it can also be implemented in the impl
fn make_rec(height:u32,width:u32) -> Rectangle{
    Rectangle{
        height,
        width
    }
}

impl Rectangle{
    // This is a proper constructor
    fn new(height:u32,width:u32) -> Self{
        Rectangle{
            height,
            width
        }
    }
    // the first parameter is a reference to self
    fn area(&self) -> u32{
        self.height * self.width
    }

    // The self parameter is not needed to be passed
    fn givDetes(&self,mes:&str){
        println!("Height: {}, Width: {}\n{}",self.height,self.width,mes);
    }
}