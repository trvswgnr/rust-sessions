use std::error::Error;

#[macro_export]
macro_rules! string_from_chars {
    ( $( $x:expr ),* ) => {{
        let mut s = String::new();
        $(
            s.push($x);
        )*
        s
    }};
}

fn main() {
    let x = mult_by_3(3).unwrap();
    let y = maybe_give_me_something().unwrap();
    println!("{y}");
    println!("{x}");
    let x = string_from_chars!('a', 'b');
    println!("{x}");
}

fn mult_by_3(x: i32) -> Result<i32, std::io::Error> {
    if x > 100 {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "sdfsd"))
    } else {
        Ok(x * 3)
    }
}

fn maybe_give_me_something() -> Result<String, Box<dyn Error>> {
    Ok(String::from("something"))
}

use std::io;

fn get_string() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn maybe_do_the_thing() -> Result<String, Box<dyn Error>> {
    let x = get_string()?;
    Ok(x)
}
