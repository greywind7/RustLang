use std::rc::Rc;
// Simple linked list, to be improved upon later
// In conclusion dont even try :)
struct Node{
    dat:i32,
    next:Option<Box<Node>>
}

impl Node{
    fn new(dat:i32) -> Self{
        Node{
            dat,
            next:None
        }
    }
}

struct List<'a>{
    head:Option<Node>,
    tail: Option<&'a Node>,
    len:i32,
    top:i32
}

impl List<'_>{
    fn new() -> Self{
        List{
            head:None,
            tail: None,
            len:0,
            top:0
        }
    }
    fn top(&self) -> i32{
        self.top
    }
    fn push(&mut self,dat:i32){
        match self.head {
            None => {let temp = Node::new(dat);
            self.head = Some(temp);
            self.tail = Some(&temp);
            self.top = dat;
            self.len+=1;}
        }
    }
}

fn main(){
    let mut a = List::new();
    println!("Data rn is {}",a.top());
    // let mut x = Box::new(tmp::new(2,2));
    // // x.a = 5;
    // x.next = Some(Box::new(tmp::new(9,4)));
    // let y = &mut x.next.unwrap();
    // // y.b = 5;
    // println!("{} {}",x.a, y.b);
    // let z = tmp::new(32, 64);
    // println!("{} {}",z.a,z.b)
}