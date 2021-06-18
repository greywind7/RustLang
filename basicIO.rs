use std::io::Write;
fn main(){
    // This is a handle for the stdin
    let reader = std::io::stdin();
    // creating a blank String
    let mut tmp = String::new();
    // The string is stored in tmp an number of bytes read is returned
    // The unwrap implicitly works with Option<R,E> enum, otherwise its needed to be defined explicitly
    let b = reader.read_line(&mut tmp).unwrap();
    parse(&tmp);

    // Similar stuff for std out
    let mut writer = std::io::stdout();
    let b = writer.write(tmp.as_bytes()).unwrap();
}

// parsing in rust is kind of a pain in the ass
// This particular parsing works only for 5 elements
// Turns out, counting the number of elements in an iterator is a pain in the ass as well :D
fn parse(yo:&String)
{
    // let yo = "1    2 3 4.3 5.12".to_string();
    // split returns an iterator
    let mut st = yo.split_whitespace();
    let mut tot:f32 = 0.0;
    // instead of next, I could have used to iterate over the iterator as well
    for i in 0..5{
        let a = st.next().unwrap();
        tot+= a.parse::<f32>().unwrap();
        println!("Number {} is {}",i,a);
    }
    println!("Heehoohoo yo total is: {}",tot);
}