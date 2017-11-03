fn() {
    let result = read_input();

    let printer = Printer{};
    printer.print_output(result.as_str());
}

struct Printer{}

impl Printer{
    fn print_output(&self, name: &str){
        println!("Hello, {}!", name);
        println!("You are a Rustacean");
    }
}

fn read_input() -> String{
    let mut buffer = String::new();
    use std::io::{self, Read};
    let result = io::stdin().read_to_string(&mut buffer);
    result.unwrap();
    return buffer;
}
