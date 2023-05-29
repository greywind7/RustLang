use std::io::Write;
fn main(){
    // This is a handle for the stdin
    let reader = std::io::stdin();
    // creating a blank String
    let mut tmp = String::new();

    // The string is stored in tmp an number of bytes read is returned
    // The unwrap implicitly works with Option<R,E> enum, otherwise its needed to be defined explicitly
    // If a returned value is successful, it stores it in the value otherwise it puts Err value in it
    // you can use expect to handle errors; if the value is successful, it is returned otherwise error message is displayed and program terminates
    // We can use match to better handle the error case instead of just terminating 
    let b = reader
            .read_line(&mut tmp)
            .expect("Could not even read");

    parse(&tmp);

    // Similar stuff for std out
    let mut writer = std::io::stdout();
    let b = writer
            .write(tmp.as_bytes())
            .expect("Could not write to the console :(");
}

fn parse(inp:&String)
{
    // let inp = "1    2 3 4.3 5.12".to_string();
    // split returns an iterator
    let iter = inp.split_whitespace();
    let mut tot:f32 = 0.0;
    let mut ctr = 1;

    for i in iter{
        tot+= match i.parse::<f32>(){
            Ok(num) => num,
			Err(_) => {
				println!("{i} doesnt seem very floaty now");
				continue;
			},
        };
        println!("Number {ctr} is {i}");
		ctr += 1;
    }
    println!("Heehoohoo yo total is: {}\n\n",tot);
}