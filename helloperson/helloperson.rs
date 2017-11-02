fn main() {
    use std::io::{self, Read};
    let mut buffer = String::new();
    let result = io::stdin().read_to_string(&mut buffer);

    if !result.is_ok() {
        return ();
    }

    let printer = Printer{};
    printer.print_output(buffer.as_str());
}

struct Printer{}

impl Printer{
    fn print_output(&self, name: &str){
        println!("Hello, {}!", name);
        println!("You are a Rustacean");
    }
}
