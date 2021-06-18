// Generics are like templates in C++
// Albeit easier to implement
struct Pair<T> {
    fir:T,
    sec:T,
}

fn make_pair<T:PartialOrd> (fir:T,sec:T)-> Pair<T>{
    Pair{
        fir,
        sec
    }
}

// Ord/PartialOrd must be specified for comparisons
impl<T:PartialOrd> Pair<T>{
    fn max (self)-> T{
        if self.fir > self.sec{
            return self.fir;
        }
        return self.sec;
    }
}

fn main(){
    let paio = make_pair(5.99,10.01);
    println!("The bigger number is {}",paio.max());
}