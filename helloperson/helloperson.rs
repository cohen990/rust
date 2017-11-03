fn main() {
    let mut buffer = String::new();
    read_input(&mut buffer);

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

fn read_input(mut buffer: &mut String){
    use std::io::{self, Read};
    let result = io::stdin().read_to_string(&mut buffer);
    result.unwrap();
}
